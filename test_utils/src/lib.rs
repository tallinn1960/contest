//! Test utilities for the `submatrix_sum` crate.

/// Create an 500 * 500 grid with pseudo-random positive i32 numbers in the range
/// 0..1000 with uniform distribution, deterministic, will always yield the same
/// grid.
pub fn generate_test_grid() -> (Vec<Vec<i32>>, i32) {
    use rand::distributions::Distribution;
    // Create an 500 * 500 grid with pseudo-random positive i32 numbers in the range
    // 0..1000 with uniform distribution, deterministic, will always yield the same
    // grid.
    let mut grid = vec![];
    let rng = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(0);
    let mut generator = rand::distributions::Uniform::new(0, 1000).sample_iter(rng);
    (0..500).for_each(|_| {
        grid.push(
            (0..500)
                .map(|_| generator.next().unwrap())
                .collect::<Vec<_>>(),
        );
    });

    // make sure that there are a sufficient number of submatrices
    let k = grid[0].iter().sum::<i32>() / 2;
    (grid, k)
}

/// Create an 500 * 500 grid with pseudo-random positive i32 numbers in the range
/// 0..1000 with uniform distribution, deterministic, will always yield the same
/// grid.
pub fn generate_test_grid_1000x1000() -> (Vec<Vec<i32>>, i32) {
    use rand::distributions::Distribution;
    // Create an 1000 * 1000 grid with pseudo-random positive i32 numbers in the range
    // 0..1000 with uniform distribution, deterministic, will always yield the same
    // grid.
    let mut grid = vec![];
    let rng = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(0);
    let mut generator = rand::distributions::Uniform::new(0, 1000).sample_iter(rng);
    (0..1000).for_each(|_| {
        grid.push(
            (0..1000)
                .map(|_| generator.next().unwrap())
                .collect::<Vec<_>>(),
        );
    });

    // make sure that there are a sufficient number of submatrices
    let k = grid[0].iter().sum::<i32>() / 2;
    (grid, k)
}
