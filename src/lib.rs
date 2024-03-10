#![allow(missing_docs)]

use std::vec;

pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
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
                acc.iter_mut()
                    .zip(row)
                    .map(|(a, r)| {
                        *a += r;
                        *a
                    })
                    .take_while(|&e| e <= k)
                    .count() as i32,
            )
        })
        .take_while(|&count_from_row| count_from_row > 0)
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
                acc.iter_mut()
                    .zip(row)
                    .map(|(a, r)| {
                        *a += r;
                        *a
                    })
                    .take_while(|&e| e <= k)
                    .count() as i32,
            )
        })
        .take_while(|&count_from_row| count_from_row > 0)
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

    use test_utils::generate_test_grid;

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
    fn test_count_submatrices_edge1() {
        let nums = vec![vec![10], vec![5]];
        let result = count_submatrices(nums, 5);
        assert_eq!(result, 0);
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

    #[test]
    fn test_implementations_with_big_grid() {
        let (grid, k) = generate_test_grid();

        println!("k: {}", k);
        println!("number of submatrices: {}", count_submatrices_ref(&grid, k));
        assert_eq!(
            count_submatrices_ref(&grid, k),
            count_submatrices_fastest(grid.clone(), k)
        );
        assert_eq!(
            count_submatrices_ref(&grid, k),
            count_submatrices(grid.clone(), k)
        );
    }


}

