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
    // Outputs
    // [0] = 0.526557
    // [1] = 0.542725
    // [2] = 0.636465
    // [3] = 0.405902
    // [4] = 0.034343
    // [5] = 0.414957
    // [6] = 0.737424
    // [7] = 0.849252
    // [8] = 0.131279
    // [9] = 0.003252
}
