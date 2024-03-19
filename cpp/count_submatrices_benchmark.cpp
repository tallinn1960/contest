#include <random>
#include <vector>
#include <benchmark/benchmark.h>

extern int count_submatrices(std::vector<std::vector<int>>& matrix, int target);

std::vector<std::vector<int>> generate_test_grid(int n) {
    // Create an n * n grid with pseudo-random positive i32 numbers in the range
    // 0..1000 with uniform distribution, deterministic, will always yield the same
    // grid.
    std::vector<std::vector<int>> grid(n, std::vector<int>(n));
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<long> dis(0, 1000);

    for (int i = 0; i < n; i++) {
        std::generate(grid[i].begin(), grid[i].end(), [&]() {
            return dis(gen);
        });
    }

    return grid;

}

auto grid = generate_test_grid(100);

// sum over the first row of grid
static int sum_first_row() {
    int sum = 0;
    for (int i = 0; i < 100; i++) {
        sum += grid[0][i];
    }
    return sum;
}

auto k = sum_first_row();

void BM_count_submatrices(benchmark::State &state) {
    for (auto _ : state)
        count_submatrices(grid, k);
}

BENCHMARK(BM_count_submatrices);

BENCHMARK_MAIN();