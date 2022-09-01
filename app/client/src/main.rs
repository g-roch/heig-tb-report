use reqwest;
use std::{thread, time};

use client::voting_choose;
use curve25519_dalek::ristretto::CompressedRistretto;
use lib::crypto::*;
use lib::VoteOption;
use lib::BOARD_PORT;
use serde_json;
use zkp;

type Round1 = (Vec<CompressedRistretto>, Vec<pr1::CompactProof>);
type Round2 = (Vec<CompressedRistretto>, Vec<pr2::CompactProof>);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rng = zkp::rand::thread_rng();

    let isopen: bool = reqwest::get(format!("http://localhost:{}/isopen", BOARD_PORT))
        .await?
        .json()
        .await?;
    if !isopen {
        panic!("Impossible de joindre une votation après la publication des clés");
    }

    let voterid: usize = reqwest::get(format!("http://localhost:{}/newvoter", BOARD_PORT))
        .await?
        .json()
        .await?;

    let vote_options: Vec<VoteOption> =
        reqwest::get(format!("http://localhost:{}/options", BOARD_PORT))
            .await?
            .json()
            .await?;
    let options: Vec<_> = vote_options.iter().map(|v| v.id as u64).collect();

    let mut crypto = Crypto::new(voterid as u64, rng.clone(), &options, options.clone());
    crypto.unset_option();

    let round_1 = crypto.vote_round_1();
    let round_1_str: String = serde_json::to_string(&round_1)?;

    let params = [("voter", voterid.to_string()), ("data", round_1_str)];
    let client = reqwest::Client::new();
    client
        .post(format!("http://localhost:{}/add-round1", BOARD_PORT))
        .form(&params)
        .send()
        .await?;

    println!("Attends l'ouverture du vote");
    while !reqwest::get(format!("http://localhost:{}/canvote", BOARD_PORT))
        .await?
        .json()
        .await?
    {
        thread::sleep(time::Duration::from_millis(100));
    }

    let choose = voting_choose(&options)?;
    let choose: Vec<_> = (0..choose.len())
        .map(|pos| choose.len() - 1 - choose.iter().position(|i| i == &&(pos as u64)).unwrap())
        .collect();
    //crypto.set_option(&choose.iter().map(|i| **i).collect());
    crypto.set_option(&choose.iter().map(|i| *i as u64).collect());
    //dbg!(choose);

    //println!("get-Round1");
    let vec_round1: Vec<Round1> =
        reqwest::get(format!("http://localhost:{}/get-round1", BOARD_PORT))
            .await?
            .json()
            .await?;
    //dbg!(vec_round1.len());
    let round_2: Round2 = crypto.vote_round_2(&vec_round1).unwrap();
    let round_2_str: String = serde_json::to_string(&round_2)?;

    //println!("add-Round2");
    let params = [("voter", voterid.to_string()), ("data", round_2_str)];
    client
        .post(format!("http://localhost:{}/add-round2", BOARD_PORT))
        .form(&params)
        .send()
        .await?;

    println!("Attends sur d'autre votant pour les résultats");
    let mut vec_round_2: Option<Vec<Round2>> = None;
    while vec_round_2.is_none() {
        vec_round_2 = reqwest::get(format!("http://localhost:{}/get-round2", BOARD_PORT))
            .await?
            .json()
            .await?;
        thread::sleep(time::Duration::from_millis(100));
    }
    let vec_round_2 = vec_round_2.unwrap();
    let tallying = crypto.tallying(&vec_round1, &vec_round_2).unwrap();

    for (pos, score) in tallying.iter().enumerate() {
        println!("{} = {score}", vote_options[pos]);
    }

    //dbg!(tallying);

    Ok(())
}
