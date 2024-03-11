# Demo for benchmarking Rust code with Criterion

Solutions for the leetcode problem [_Count submatrices with Top-Left Element and Sum Less Than k_](https://leetcode.com/problems/count-submatrices-with-top-left-element-and-sum-less-than-k/description/) are compared in 
performance using the Criterion benchmark framework.

# Measurement

The repository contains a Criterion benchmark to measure the runtime of several functions, which
all do the same on the same input (calculation numbers of submatrices):

- *mine/cs* 

consumes an 500x500 Vec<Vec<i32>> field with random integers and a sufficiently
large submatrix sum limit to finde a substantial amount of submatrices.

- *mine/cs1000x1000*

same algorithm consuming a 1000x1000 field.

- *mine/ref*

some implementation of the algorithm, same input, but input is passed as reference.
Appears to be considerably faster.

- *mine/fs*

a variant of the algorithm producing a list of submatrices containing their sum and
the x,y position of the lower end corner of the submatrix. _example/main_ contains
an example program visualising the output as a png image file.

- *leetcode/cs*

the fastest solution for computing submatrices I found on leetcode, consumes its input

- *leetcode/cs_unchecked*

an unsafe version of _leetcode/cs_ that uses unchecked indexed vector access, much 
faster than leetcode/cs

- *leetcode/cs_raw_ptr*

unsafe as _leetcode/cs_, bet using raw ptr access. Surprisingly no gain in performance
compared to _leetcode/cs_.

All functions are measured with exactly the same input. Run the benchmarks with

```
cargo bench
```

Individual benchmark can be run in isolation by giving cargo bench their name.

# Note
In developing this repo, it was found that Criterion produces unreliable 
results if the functions tested have a runtime in the range of µs or lower.
To get reliable results consistent on all platforms one needs to increase the
dataset input size (assuming that runtime depends on it) to take the execution
time of one function call near or into the range of ms.

# Acknowledgements

*patryk27* on *Reddit* for helping me figuring out misleading performance measurements
on different platforms.

*dacozai* on *leetcode* for the contribution of algorithm versions using indices.

# Further reading

The *mine/cs* solution and why it is the fastest is explained in detail on [leetcode](https://leetcode.com/problems/count-submatrices-with-top-left-element-and-sum-less-than-k/solutions/4849239/the-return-of-the-iterators-or-why-it-is-good-to-be-lazy).