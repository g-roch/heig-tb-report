use lib::crypto::*;

fn main() {
    let rng = zkp::rand::thread_rng();
    let options = vec![0, 1, 2, 3];

    let chooses = vec![
        vec![0, 3, 1, 2],
        vec![0, 2, 1, 3],
        vec![0, 2, 1, 3],
        vec![0, 2, 1, 3],
        vec![0, 2, 1, 3],
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
    let expected: Vec<_> = (0..options.len())
        .map(|i| chooses.iter().map(|v| v[i]).sum())
        .collect();
    dbg!(&expected);
    assert_eq!(r, expected);

    //crypto.vote();
}
