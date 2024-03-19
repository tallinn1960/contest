# Demo for benchmarking Rust code with Criterion

Solutions for the leetcode problem [_Count submatrices with Top-Left Element and Sum Less Than k_](https://leetcode.com/problems/count-submatrices-with-top-left-element-and-sum-less-than-k/description/) are compared in 
performance using the Criterion benchmark framework.

# Measurement

The repository contains a Criterion benchmark to measure the runtime of several functions, which
all do the same on the same input (calculation numbers of submatrices):

- *mine/cs* 

consumes an 500x500 `Vec<Vec<i32>>` field with random integers and a sufficiently
large submatrix sum limit to finde a substantial amount of submatrices.

- *mine/cs1000x1000*

same algorithm consuming a 1000x1000 field.

- *mine/ref*

same implementation of the algorithm, same input, but input is passed as reference.
Appears to be considerably faster.

- *mine/fs*

a variant of the algorithm producing a list of submatrices containing their *sum* and
the *x, y* position of the lower right end corner of the submatrix. _example/main_ contains
an example program visualising the output as a png image file.

- *leetcode/cs*

the fastest solution for computing submatrices I found on leetcode, consumes its input

- *leetcode/cs_unchecked*

an unsafe version of _leetcode/cs_ that uses unchecked indexed vector access, much 
faster than *leetcode/cs*

- *leetcode/cs_raw_ptr*

unsafe as *leetcode/cs_unchecked*, but using raw ptr access. Surprisingly no gain in performance
compared to _leetcode/cs_.

All functions are measured with exactly the same input. Run the benchmarks with

```
cargo bench
```

Individual benchmark can be run in isolation by giving cargo bench their name.

# C++ benchmark

There is a c++ variant mimicing the Rust solution and a benchmark using the
Google benchmark library in the subfolder *cpp*. Build and run it with *cmake* (but make sure to run the benchmark as a release build). On *clang* and Apple Silicon, c++ is still five times slower than the Rust code but rumours has it that llvm isn't as good in optimizing arm64 code yet as it is on other architectures. Note that you need to compare the time of the c++ solution to the *mine/ref* variant of the Rust benchmark, as the c++ solution takes the test data by reference as well.

# Note
In developing this repo, it was found that Criterion produces unreliable 
results if the functions tested have a runtime in the range of a few µs or lower.
To get reliable results consistent on all platforms one needs to increase the
dataset input size (assuming that runtime depends on it) to take the execution
time of one function call near or into the range of about 100 µs or higher. This
may as well depend on the machine you are using for testing. But no implementation
has a built-in limit for input dataset size and should work with all sizes, as
long as no `i32` sum overflow happens.

# Acknowledgements

*patryk27* on *Reddit* for helping me figuring out misleading performance measurements
on different platforms.

*dacozai* on *leetcode* for the contribution of algorithm versions using indices.

# Further reading

The *mine/cs* solution and why it is the fastest is explained in detail on [leetcode](https://leetcode.com/problems/count-submatrices-with-top-left-element-and-sum-less-than-k/solutions/4849239/the-return-of-the-iterators-or-why-it-is-good-to-be-lazy).