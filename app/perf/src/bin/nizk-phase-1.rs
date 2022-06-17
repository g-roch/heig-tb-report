fn main() {
    use curve25519_dalek::ristretto::CompressedRistretto;
    use lib::crypto::pr1;
    use lib::crypto::Crypto;
    use rayon::prelude::*;

    let options = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let n = 80_000;

    let mut round_1: Vec<_> = (1..n as u64)
        .into_par_iter()
        .map(|i_voterid| {
            let rng = zkp::rand::thread_rng();
            let mut crypto = Crypto::new(i_voterid, rng, &options, options.clone());
            crypto.vote_round_1()
        })
        .collect();

    let rng = zkp::rand::thread_rng();
    let mut crypto = Crypto::new(0, rng, &options, options.clone());
    round_1.insert(0, crypto.vote_round_1());

    Crypto::<zkp::rand::rngs::ThreadRng>::verify_round_1(&round_1);

    println!("{}", round_1.len());
}
