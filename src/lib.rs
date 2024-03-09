#![allow(missing_docs)]

use std::vec;

pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = grid.first().unwrap().len();
     grid.into_iter()
        .map(|row| {
            row.into_iter().scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
        })
        .scan(vec![0; n], |acc, row| {
            Some(
                acc.into_iter()
                    .zip(row)
                    .map(|(x, y)| {
                        *x += y;
                        *x
                    })
                    .filter(|&x| x <= k)
                    .count() as i32,
            )
        })
        .sum::<i32>()
}

pub fn count_submatrices_ref(grid: &Vec<Vec<i32>>, k: i32) -> i32 {
    let n = grid.first().unwrap().len();
    grid.iter()
        .map(|row| {
            row.iter().scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
        })
        .scan(vec![0; n], |acc, row| {
            Some(
                acc.into_iter()
                    .zip(row)
                    .map(|(x, y)| {
                        *x += y;
                        *x
                    })
                    .filter(|&x| x <= k)
                    .count() as i32,
            )
        })
        .sum::<i32>()
}

pub fn count_submatrices_fastest(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut ans = 0i32;
    for i in 0..grid.len() {
        for j in 1..grid[i].len() {
            grid[i][j] += grid[i][j - 1];
        }
    }

    for c in 0..grid[0].len() {
        let mut sumup = 0i32;
        for r in 0..grid.len() {
            sumup += grid[r][c];
            if sumup <= k {
                ans += 1;
            } else {
                break;
            }
        }
    }

    ans
}


#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_count_submatrices() {
        let nums = vec![vec![7, 6, 3], vec![6, 6, 1]];
        let result = count_submatrices(nums, 18);
        assert_eq!(result, 4);
        let nums = vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]];
        let result = count_submatrices(nums, 20);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_count_submatrices_ref() {
        let nums = vec![vec![7, 6, 3], vec![6, 6, 1]];
        let result = count_submatrices_ref(&nums, 18);
        assert_eq!(result, 4);
        let nums = vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]];
        let result = count_submatrices_ref(&nums, 20);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_count_submatrices_fastest() {
        let nums = vec![vec![7, 6, 3], vec![6, 6, 1]];
        let result = count_submatrices_fastest(nums, 18);
        assert_eq!(result, 4);
        let nums = vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]];
        let result = count_submatrices_fastest(nums, 20);
        assert_eq!(result, 6);
    }
}
