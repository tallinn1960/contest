//! # Submatrices with Sum Less Than K

use std::vec;

/// Given a non-empty 2D matrix grid of non-negative integers `grid`, an integer `k`, 
/// find the maximum number of submatrices such that the sum of the elements 
/// inside the submatrix is less than or equal to `k`.
/// 
/// ## Constraints
/// * `0 <= grid[i][j] <= 1000`
/// * `0 <= k <= 10^6`
/// 
/// ## Examples
/// 
/// ```rust
/// use contest::count_submatrices;
/// 
/// let nums = vec![vec![7, 6, 3], vec![6, 6, 1]];
/// let result = count_submatrices(nums, 18);
/// assert_eq!(result, 4);
/// 
/// let nums = vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]];
/// let result = count_submatrices(nums, 20);
/// assert_eq!(result, 6);
/// ```
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

/// Given a non-empty 2D matrix grid of non-negative integers `grid`, an integer `k`,
/// find the submatrices such that the sum of the elements inside the submatrix is 
/// less than or equal to `k`. Return the sum of the submatrices, the x and y coordinates
/// of the lower right corner of the submatrix. All submatrices have their upper left
/// corner at (0, 0).
/// 
/// ## Constraints
/// 
/// * `0 <= grid[i][j] <= 1000`
/// * `0 <= k <= 10^6`
/// 
/// ## Examples
/// 
/// ```rust
/// use contest::find_submatrices;
/// 
/// let nums = vec![vec![7, 6, 3], vec![6, 6, 1]];
/// let result = find_submatrices(&nums, 18);
/// assert_eq!(result, vec![(7, 0, 0), (13, 1, 0), (16, 2, 0), (13, 0, 1)]);
/// 
/// let nums = vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]];
/// let result = find_submatrices(&nums, 20);
/// assert_eq!(result, vec![(7, 0, 0), (9, 1, 0), (18, 2, 0), (8, 0, 1), (15, 1, 1), (10, 0, 2)]);
/// ```
pub fn find_submatrices(grid: &Vec<Vec<i32>>, k: i32) -> Vec<(i32, usize, usize)> {
    let n = grid.first().unwrap().len();
    grid.iter()
        .map(|row| {
            row.iter().scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
        })
        .enumerate()
        .scan(vec![0; n], |acc, row| {
            Some(
                acc.iter_mut()
                    .zip(row.1)
                    .enumerate()
                    .map(|(x, (a, r))| {
                        *a += r;
                        (*a, x, row.0)
                    })
                    .take_while(|&e| e.0 <= k)
                    .collect::<Vec<(i32, usize, usize)>>()
            )
        })
        .flatten()
        .collect::<Vec<_>>()
}

/// Ref version of count_submatrices
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

/// Fastest version of count_submatrices from leetcode
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

#[allow(unsafe_code)]
/// Unsafe version of count_submatrices from leetcode
pub fn count_submatrices_unchecked(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
    for i in 0..grid.len() {
        unsafe {
            let row = grid.get_mut(i).unwrap();
            let row_ptr = row.as_mut_ptr();
            for j in 1..grid[i].len() {
                *row_ptr.add(j) += *row_ptr.add(j-1);
            }
        }
    }

    let mut ans = 0i32;
    for c in 0..grid[0].len() {
        let mut sumup = 0i32;
        for r in 0..grid.len() {
            unsafe {
                let row = grid.get_mut(r).unwrap();
                let row_ptr = row.as_mut_ptr();
                sumup += *row_ptr.add(c);
                if sumup <= k {
                    ans += 1;
                } else {
                    break;
                }
            }
        }
    }

    ans
}

#[allow(unsafe_code)]
/// Raw pointer version of count_submatrices from leetcode
pub fn count_submatrices_raw_ptr(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let row_len = grid.len();
    let col_len = grid[0].len();
    let vv = grid.as_mut_ptr();

    for i in 0..row_len {
        unsafe {
            let row_ptr = vv.add(i).as_mut().unwrap();
            for j in 1..col_len {
                let val_ptr = row_ptr.as_mut_ptr(); 
                *val_ptr.add(j) += *val_ptr.add(j-1);
            }
        }
    }    


    let mut ans = 0i32;
    for c in 0..col_len {
        let mut sumup = 0i32;
        for r in 0..row_len {
            unsafe {
                let row_ptr = vv.add(r).as_mut().unwrap();
                let val_ptr = row_ptr.as_mut_ptr();
                sumup += *val_ptr.add(c);
            }
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
