#include <cassert>
#include <ranges>


int count_submatrices(std::vector<std::vector<int>>& matrix, int target) {
    int n = matrix.size();
    auto v = matrix | std::ranges::views::transform([](auto& row) {
        for (int i = 1; i < row.size(); ++i) {
            row[i] += row[i - 1];
        }
        return row;
    });
    return 0;
}

int main() {
    std::vector<std::vector<int>> matrix = {{0, 1, 0}, {1, 1, 1}, {0, 1, 0}};
    int target = 0;
    auto v = count_submatrices(matrix, target);
    assert(v == 0);
}