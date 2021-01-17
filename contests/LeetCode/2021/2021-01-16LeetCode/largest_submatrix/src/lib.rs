/*
You are given a binary matrix matrix of size m x n, and you are allowed to rearrange the columns of
the matrix in any order.

Return the area of the largest submatrix within matrix where every element of the submatrix is 1
after reordering the columns optimally.
*/
use std::cmp::{self, Reverse};

// O(mnlog(m)) solution
pub fn largest_submatrix(mut matrix: Vec<Vec<i32>>) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut res = 0;

    // at each index, store the number of consecutive 1s above and including it
    for j in 0..cols {
        for i in 1..rows {
            if matrix[i][j] == 1 {
                matrix[i][j] += matrix[i - 1][j];
            }
        }
    }
    // sort each row in descending order to determine the area of the tallest rectangle including
    // the current index
    for i in 0..rows {
        matrix[i].sort_unstable_by_key(|&val| Reverse(val));
        for j in 0..cols {
            res = cmp::max(res, (j + 1) as i32 * matrix[i][j]);
        }
    }
    res
}

// O(m*n^2) implementation
// pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
//     let rows = matrix.len();
//     let cols = matrix[0].len();
//     let mut dp = matrix.clone();
//     let mut res = 0;
//     for k in 0..rows {
//         for i in 0..rows - k {
//             for j in 0..cols {
//                 dp[i][j] &= dp[i + k][j];
//             }
//             res = cmp::max(res, dp[i].iter().sum::<i32>() * (k + 1) as i32);
//         }
//     }
//     res
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
