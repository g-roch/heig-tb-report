fn main() {
    use curve25519_dalek::constants as dalek_constants;
    use curve25519_dalek::scalar::Scalar;
    use std::time::Instant;

    let start = Instant::now();
    let g = &dalek_constants::RISTRETTO_BASEPOINT_POINT;
    let v = dalek_constants::RISTRETTO_BASEPOINT_POINT;
    let mut i = 2u64;
    let mut a = Scalar::from(i) * g;
    loop {
        a += g;
        i += 1;
        if a == v {
            eprintln!("Result is {i}");
            break;
        }

        if 0 == i % 1_000_000 {
            let duration = Instant::now() - start;
            println!("{:0.2}M exp en {duration:?}", i as f64 / 1_000_000f64);
        }
    }
}
