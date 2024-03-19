#include <vector>

int count_submatrices(std::vector<std::vector<int>>& matrix, int target) {
    int count = 0;
    std::vector<int> submatrix_sums(matrix[0].size(), 0);
    for (auto r : matrix) {
        int submatrices_in_row = 0;
        submatrix_sums[0] += r[0];
        if (submatrix_sums[0] > target) {
            break;
        }
        submatrices_in_row++;
        for (int i = 1; i < r.size(); i++) {
            r[i] += r[i - 1];
            submatrix_sums[i]+= r[i];
            if (submatrix_sums[i] > target) {
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