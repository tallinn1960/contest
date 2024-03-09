#![allow(missing_docs)]

use contest::{count_submatrices, count_submatrices_fastest, count_submatrices_ref};
use criterion::{criterion_group, criterion_main, Criterion};

fn count_submatrices_benchmark(c: &mut Criterion) {
    let grid = vec![vec![7, 6, 3], vec![6, 6, 2]];
    c.bench_function("count_submatrices", |b| {
        b.iter_batched(
            || grid.clone(),
            |t| count_submatrices(t, 18),
            criterion::BatchSize::SmallInput,
        )
    });
    c.bench_function("count_submatrices_ref", |b| {
        b.iter(|| count_submatrices_ref(&grid, 18))
    });
    c.bench_function("count_submatrices_fastest", |b| {
        b.iter_batched(
            || grid.clone(),
            |t| count_submatrices_fastest(t, 18),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, count_submatrices_benchmark);
criterion_main!(benches);
