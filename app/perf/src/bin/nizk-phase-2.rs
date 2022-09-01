fn main() {
    use curve25519_dalek::constants as dalek_constants;
    use curve25519_dalek::ristretto::CompressedRistretto;
    use curve25519_dalek::ristretto::RistrettoPoint;
    use curve25519_dalek::scalar::Scalar;
    use lib::crypto::pr2;
    use lib::crypto::Crypto;
    use rayon::prelude::*;
    use sha2::Sha512;
    use std::time::Instant;
    use zkp::rand::rngs::ThreadRng;
    use zkp::Transcript;

    let options: Vec<_> = (0..30).collect();
    let k = options.len();
    let rng = zkp::rand::thread_rng();
    let mut crypto0 = Crypto::new(0, rng, &options, options.clone());
    let mut crypto1 = Crypto::new(1, rng, &options, options.clone());
    let mut crypto2 = Crypto::new(2, rng, &options, options.clone());
    let round1 = vec![
        crypto0.vote_round_1(),
        crypto1.vote_round_1(),
        crypto2.vote_round_1(),
    ];

    let round2 = crypto1.vote_round_2(&round1).unwrap();

    let start = Instant::now();

    let G = &dalek_constants::RISTRETTO_BASEPOINT_POINT;
    let A: Vec<CompressedRistretto> = options
        .iter()
        .map(|opt| Scalar::from(*opt) * G)
        .map(|p| p.compress())
        .collect();
    let G = &dalek_constants::RISTRETTO_BASEPOINT_COMPRESSED;

    for i in 0u64.. {
        let i_voterid = 1usize;
        let ID = RistrettoPoint::hash_from_bytes::<Sha512>(&i_voterid.to_le_bytes()).compress();
        let Y: Vec<_> = Crypto::<ThreadRng>::calc_Yi(&round1, i_voterid)
            .unwrap()
            .iter()
            .map(RistrettoPoint::compress)
            .collect();
        round2
            .1
            .iter()
            .enumerate()
            .map(|(j, proof)| {
                pr2::verify_compact(
                    &proof,
                    pr2::VerifyAssignments {
                        ID: &ID,
                        X: &round1[i_voterid].0,
                        Y: &Y,
                        G: &G,
                        A: &A[j],
                        Z: &round2.0,
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        if 0 == i % 100 {
            let duration = Instant::now() - start;
            println!("{i} x {k} NIZK 2 verification en {duration:?}");
        }
    }
}
