use lib::crypto::*;
use zkp::rand::prelude::*;

fn main() {
    let mut rng = zkp::rand::thread_rng();
    let options = vec![0, 1, 2];

    let chooses = vec![
        vec![2, 0, 1],
        vec![0, 2, 1],
        vec![0, 2, 1],
        vec![0, 2, 1],
        vec![0, 2, 1],
    ];

    let mut cryptos: Vec<_> = chooses
        .iter()
        .enumerate()
        .map(|(i_voterid, choose)| {
            Crypto::new(i_voterid as u64, rng.clone(), &options, choose.clone())
        })
        .collect();

    let round_1: Vec<_> = cryptos
        .iter_mut()
        .map(|crypto| crypto.vote_round_1())
        .collect();

    let round_2: Result<Vec<_>, _> = cryptos
        .iter()
        .map(|crypto| crypto.vote_round_2(&round_1))
        .collect();

    let tallying = cryptos[0].tallying(&round_1, &round_2.unwrap());

    dbg!(&tallying);

    let r = tallying.unwrap();
    dbg!(r == vec![2, 8, 5]);

    //crypto.vote();
}
