use rand::Rng;

pub fn inspect_seed(seed: u64, count: usize) {
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    let mut rng = StdRng::seed_from_u64(seed);
    println!("Seed {} produces:", seed);

    for i in 0..count {
        let val: f64 = rng.random();
        println!("  [{}] = {:.6}", i, val);
    }
}

pub fn inspect_seed_for_d20(seed: u64, count: usize) {
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    let mut rng = StdRng::seed_from_u64(seed);
    println!("Seed {} produces:", seed);

    for i in 0..count {
        let val = rng.random_range(1..=20);
        println!("{} {}", i, val);
    }
}
