#![allow(missing_docs)]

use contest::{count_submatrices, count_submatrices_fastest, count_submatrices_ref};
use criterion::{criterion_group, criterion_main, Criterion};

fn count_submatrices_benchmark(c: &mut Criterion) {
    // create an 1200 * 1400 grid with random positive i32 numbers

    let mut grid = vec![];
    let k = i32::MAX / 2; // let's use a big number to make the benchmark more interesting

    (0..1200).for_each(|_| {
        grid.push(
            (0..1400)
                .map(|_| rand::random::<i32>().abs() / 10000)
                .collect::<Vec<_>>(),
        );
    });

    let mut group = c.benchmark_group("count_submatrices");
    group.measurement_time(std::time::Duration::from_secs(6));

    group.bench_function("count_submatrices", |b| {
        b.iter_batched(
            || grid.clone(),
            |t| count_submatrices(t, k),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("count_submatrices_ref", |b| {
        b.iter(|| count_submatrices_ref(&grid, k))
    });
    group.bench_function("count_submatrices_fastest", |b| {
        b.iter_batched(
            || grid.clone(),
            |t| count_submatrices_fastest(t, k),
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, count_submatrices_benchmark);
criterion_main!(benches);
