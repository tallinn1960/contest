#![allow(missing_docs)]

use contest::{count_submatrices, count_submatrices_fastest, count_submatrices_ref};
use criterion::{criterion_group, criterion_main, Criterion};

fn count_submatrices_benchmark(c: &mut Criterion) {
    // create an 12 * 14 grid with random numbers

    let mut grid = vec![];
    let k = i32::MAX / 2;

    (0..1200).for_each(|_| {
        grid.push((0..1400).map(|_| rand::random::<i32>().abs() / 10000).collect::<Vec<_>>());
    });

    c.bench_function("count_submatrices", |b| {
        b.iter_batched(
            || grid.clone(),
            |t| count_submatrices(t, k),
            criterion::BatchSize::SmallInput,
        )
    });
    c.bench_function("count_submatrices_ref", |b| {
        b.iter(|| count_submatrices_ref(&grid, k))
    });
    c.bench_function("count_submatrices_fastest", |b| {
        b.iter_batched(
            || grid.clone(),
            |t| count_submatrices_fastest(t, k),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, count_submatrices_benchmark);
criterion_main!(benches);
