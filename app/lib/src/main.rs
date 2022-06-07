use lib::crypto::*;
use zkp::rand::prelude::*;

fn main() {
    let mut rng = zkp::rand::thread_rng();
    let options = vec!["Toto".to_string(), "Tata".to_string(), "Riri".to_string()];

    let mut cryptos: Vec<_> = (0..10)
        .map(|i| Crypto::new(i, rng.clone(), &options, vec![0, 2, 1]))
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

    dbg!(tallying);

    //crypto.vote();
}
