fn main() {
    use curve25519_dalek::constants as dalek_constants;
    use curve25519_dalek::scalar::Scalar;
    use std::time::Instant;

    let start = Instant::now();
    let g = &dalek_constants::RISTRETTO_BASEPOINT_POINT;
    let v = dalek_constants::RISTRETTO_BASEPOINT_POINT;
    for i in 2u64.. {
        if Scalar::from(i) * g == v {
            eprintln!("Ord is {i}");
            break;
        }

        if 0 == i % 10_000 {
            let duration = Instant::now() - start;
            println!("{:0.2}M exp en {duration:?}", i as f64 / 1_000_000f64);
        }
    }
}
