fn main() {
    use curve25519_dalek::constants as dalek_constants;
    use curve25519_dalek::ristretto::CompressedRistretto;
    use curve25519_dalek::ristretto::RistrettoPoint;
    use lib::crypto::pr1;
    use lib::crypto::Crypto;
    use rayon::prelude::*;
    use sha2::Sha512;
    use std::time::Instant;
    use zkp::Transcript;

    let options: Vec<_> = (0..30).collect();
    let k = options.len();
    let rng = zkp::rand::thread_rng();
    let mut crypto = Crypto::new(0, rng, &options, options.clone());
    let votant = crypto.vote_round_1();
    let start = Instant::now();

    let G = &dalek_constants::RISTRETTO_BASEPOINT_COMPRESSED;
    for i in 0u64.. {
        let ID = RistrettoPoint::hash_from_bytes::<Sha512>(&0usize.to_le_bytes()).compress();
        votant
            .0
            .iter()
            .zip(votant.1.iter())
            .enumerate()
            .map(|(j, (X, proof))| {
                let mut transcript = Transcript::new(b"prove1");
                transcript.append_u64(b"j", j as u64);
                pr1::verify_compact(
                    &proof,
                    &mut transcript,
                    pr1::VerifyAssignments { ID: &ID, X, G },
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        if 0 == i % 1000 {
            let duration = Instant::now() - start;
            println!("{i} x {k} NIZK 1 verification en {duration:?}");
        }
    }
}
