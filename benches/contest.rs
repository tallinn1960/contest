#![allow(missing_docs)]

use contest::{count_submatrices, count_submatrices_fastest, count_submatrices_raw_ptr, count_submatrices_ref, count_submatrices_unchecked, find_submatrices};
use criterion::{criterion_group, criterion_main, Criterion};
use test_utils::generate_test_grid;

fn count_submatrices_benchmark(c: &mut Criterion) {
    let (grid, k) = generate_test_grid();
    let (grid1000x1000, k1000x1000) = test_utils::generate_test_grid_1000x1000();

    println!("k: {}", k);
    println!("number of submatrices: {}", count_submatrices_ref(&grid, k));

    println!("k1000x1000: {}", k1000x1000);
    println!("number of submatrices: {}", count_submatrices_ref(&grid1000x1000, k1000x1000));
    
    let mut group = c.benchmark_group("all");
    group.measurement_time(std::time::Duration::from_secs(20));

    group.bench_function("mine/cs", |b| {
        b.iter_batched(
            || grid.clone(),
            |t| count_submatrices(t, k),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("mine/cs1000x1000", |b| {
        b.iter_batched(
            || grid1000x1000.clone(),
            |t| count_submatrices(t, k1000x1000),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("mine/ref", |b| {
        b.iter(|| count_submatrices_ref(&grid, k))
    });

    group.bench_function("mine/fs", |b| {
        b.iter(|| find_submatrices(&grid, k))

    });

    group.bench_function("leetcode/cs", |b| {
        b.iter_batched(
            || grid.clone(),
            |t| count_submatrices_fastest(t, k),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("leetcode/cs_unchecked", |b| {
        b.iter_batched(
            || grid.clone(),
            |t| count_submatrices_unchecked(t, k),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("leetcode/cs_raw_ptr", |b| {
        b.iter_batched(
            || grid.clone(),
            |t| count_submatrices_raw_ptr(t, k),
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, count_submatrices_benchmark);
criterion_main!(benches);
