# Demo for macOS/Linux arm64 performance differences

This repository contains a demo for mysterious performance differences between Rust code
compiled on either macOS or Linux (in a Parallels VM) on the same machine, a Mac mini M1.

# Measurement

The repository contains a Criterion benchmark to measure the runtime of three functions, which
all do the same on the same input (calculation numbers of submatrices):

- compute_submatrices 

consumes an Vec<Vec<i32>> field.

- compute_submatrices_ref

some implementation of the algorithm, same input, but input is passed as reference.

- compute_submatrices_fastet

the fastest solution for computing submatrices I found on leetcode, consumes its input

All functions are measured with exactly the same input. Run the benchmarks with

```
cargo bench
```

once on macOS, then on the same machine on Linux (docker or VM).

# Observation

These are the measurements on my machine for macOS (Mac mini M1, MacOS Sonoma 14.4, XCode 15.2, rust 1.76):

```
     Running benches/contest.rs (target/release/deps/contest-eea34950ff8b3d20)
count_submatrices       time:   [120.82 ns 122.94 ns 125.13 ns]

count_submatrices_ref   time:   [24.583 ns 24.644 ns 24.712 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

count_submatrices_fastest
                        time:   [108.99 ns 111.52 ns 113.83 ns]
```

and this are the measurements on the same machine running Ubuntu 23.10 with rust 1.76 and gcc 13.2:
```
     Running benches/contest.rs (target/release/deps/contest-a8573380fa1b73e5)
count_submatrices       time:   [54.562 ns 54.795 ns 55.107 ns]
                        change: [-15.529% -1.2341% +15.075%] (p = 0.82 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low severe
  4 (4.00%) high severe

count_submatrices_ref   time:   [22.952 ns 23.062 ns 23.191 ns]
                        change: [-0.6599% -0.0060% +0.6744%] (p = 0.98 > 0.05)
                        No change in performance detected.
Found 29 outliers among 100 measurements (29.00%)
  13 (13.00%) low severe
  6 (6.00%) low mild
  4 (4.00%) high mild
  6 (6.00%) high severe

count_submatrices_fastest
                        time:   [50.307 ns 50.547 ns 50.867 ns]
                        change: [-1.4050% +0.1459% +1.6936%] (p = 0.86 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low severe
  1 (1.00%) high mild
  2 (2.00%) high severe
```

As one can see, as indicated by Criterion the two functions
consuming their input are twice as fast on Linux then they are on macOS - on
the same hardware.

**Why is that?**