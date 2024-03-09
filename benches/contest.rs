#![allow(missing_docs)]

use contest::{count_submatrices, count_submatrices_fastest, count_submatrices_ref};
use criterion::{criterion_group, criterion_main, Criterion};
use rand::distributions::Distribution;

fn count_submatrices_benchmark(c: &mut Criterion) {
    // create an 1000 * 1000 grid with random positive i32 numbers in the range
    // 0..1000

    let mut grid = vec![];
    let k = 500000000; // let's use a big number to make the benchmark more interesting
    let rng = rand::thread_rng();
    let mut generator = rand::distributions::Uniform::new(0, 1000).sample_iter(rng);

    (0..500).for_each(|_| {
        grid.push(
            (0..500)
                .map(|_| generator.next().unwrap())
                .collect::<Vec<_>>(),
        );
    });

    // the input data is maxed to half the limits on each input according to leetcode constraints

    let mut group = c.benchmark_group("all");
    group.measurement_time(std::time::Duration::from_secs(20));

    group.bench_function("mine/cs", |b| {
        b.iter_batched(
            || grid.clone(),
            |t| count_submatrices(t, k),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("mine/ref", |b| {
        b.iter(|| count_submatrices_ref(&grid, k))
    });
    group.bench_function("leetcode/cs", |b| {
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
