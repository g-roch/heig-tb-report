fn main() {
    use curve25519_dalek::ristretto::CompressedRistretto;
    use lib::crypto::pr1;
    use lib::crypto::pr2;
    use lib::crypto::Crypto;
    use si_scale::helpers::bibytes;
    use si_scale::helpers::bibytes1;
    use std::mem::size_of;

    let grp: Vec<(&'static str, usize)> = vec![
        ("ASSO ", 300),
        ("LS_p ", 140_202),
        ("MIG_v", 630_000),
        ("MIG_i", 2_280_000),
        ("FR_v ", 35_923_707),
        ("FR_i ", 48_747_876),
        ("FR_p ", 68_014_000),
        ("EU_p ", 743_000_000),
        ("TER_p", 7_874_966_000),
    ];

    let options = 10;

    println!("=== 1er tour ===");
    let basic_size_1 = size_of::<(CompressedRistretto, pr1::CompactProof)>();
    println!("      {}/electeur/option", bibytes1(basic_size_1 as f64));
    let size_by_electeur = basic_size_1 * options;
    println!("      {}/electeur", bibytes1(size_by_electeur as f64));

    for (name, n) in &grp {
        let total_size = size_by_electeur * n;
        println!("{name} {}", bibytes1(total_size as f64));
    }

    println!("=== 2nd tour ===");
    let basic_size_2 = size_of::<(CompressedRistretto, pr2::CompactProof)>();
    println!("      {}/electeur/option", bibytes1(basic_size_2 as f64));
    let size_by_electeur = basic_size_2 * options;
    println!("      {}/electeur", bibytes1(size_by_electeur as f64));

    for (name, n) in &grp {
        let total_size = size_by_electeur * n;
        println!("{name} {}", bibytes1(total_size as f64));
    }

    println!("=== total ===");
    println!(
        "      {}/electeur/option",
        bibytes1((basic_size_1 + basic_size_2) as f64)
    );
    let size_by_electeur = (basic_size_1 + basic_size_2) * options;
    println!("      {}/electeur", bibytes1(size_by_electeur as f64));

    for (name, n) in &grp {
        let total_size = size_by_electeur * n;
        println!("{name} {}", bibytes1(total_size as f64));
    }
}
