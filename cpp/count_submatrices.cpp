#include <vector>

int count_submatrices(std::vector<std::vector<int>>& matrix, int target) {
    int count = 0;
    std::vector<int> row_sum(matrix[0].size(), 0);
    for (auto r : matrix) {
        int submatrices_in_row = 0;
        row_sum[0] += r[0];
        if (row_sum[0] > target) {
            break;
        }
        submatrices_in_row++;
        for (int i = 1; i < r.size(); i++) {
            r[i] += r[i - 1];
            row_sum[i]+= r[i];
            if (row_sum[i] > target) {
                break;
            }
            submatrices_in_row++;
        }
        if (submatrices_in_row == 0) {
            break;
        }
        count += submatrices_in_row;
    }
    return count;
}