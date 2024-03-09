#![allow(missing_docs)]

use contest::{count_submatrices, count_submatrices_fastest};
fn main() {
    // measure execution time of count_submatrices and count_submatrices_fastest

    let grid = vec![vec![7, 6, 3], vec![6, 6, 2]];

    let grid1 = grid.clone();
    let grid2 = grid.clone();

    let start = std::time::Instant::now();
    let r1 = count_submatrices(grid1, 18);
    let duration = start.elapsed();
    println!("count_submatrices: {} in {:?}", r1, duration);

    let start = std::time::Instant::now();
    let r2 = count_submatrices_fastest(grid2, 18);
    let duration = start.elapsed();
    println!("count_submatrices_fastest: {} in {:?}", r2, duration);

    if r1 != r2 {
        panic!("results do not match");
    }
}
