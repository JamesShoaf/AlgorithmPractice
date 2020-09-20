/* 
You are given a rows x cols matrix grid. Initially, you are located at the top-left corner (0, 0), and in each step, you can only move right or down in the matrix.

Among all possible paths starting from the top-left corner (0, 0) and ending in the bottom-right corner (rows - 1, cols - 1), find the path with the maximum non-negative product. The product of a path is the product of all integers in the grid cells visited along the path.

Return the maximum non-negative product modulo 109 + 7. If the maximum product is negative return -1.

Notice that the modulo is performed after getting the maximum product.
*/
use std::cmp;

pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
    // 2 rows of (max, min)
    let mut dp: Vec<Vec<(i32, i32)>> = vec![vec![(1, 1); grid[0].len()]; 2];
    // seed initial row
    dp[0][0] = (grid[0][0], grid[0][0]);
    for col in 1..grid[0].len() {
        let val = grid[0][col];
        dp[0][col] = (
            cmp::max(val * dp[0][col - 1].0, val * dp[0][col - 1].1),
            cmp::min(val * dp[0][col - 1].0, val * dp[0][col - 1].1),
        );
    }
    for row in 1..grid.len() {
        let val = grid[row][0];
        dp[row % 2][0] = (
            cmp::max(val * dp[(row + 1) % 2][0].0, val * dp[(row + 1) % 2][0].1),
            cmp::min(val * dp[(row + 1) % 2][0].0, val * dp[(row + 1) % 2][0].1),
        );
        for col in 1..grid[row].len() {
            let val = grid[row][col];
            let max_neighbor = cmp::max(dp[row % 2][col - 1].0, dp[(row + 1) % 2][col].0);
            let min_neighbor = cmp::min(dp[row % 2][col - 1].1, dp[(row + 1) % 2][col].1);
            dp[row % 2][col] = (
                cmp::max(val * max_neighbor, val * min_neighbor),
                cmp::min(val * max_neighbor, val * min_neighbor),
            );
        }
    }
    let best = dp[(grid.len() - 1) % 2][grid[0].len() - 1].0;
    return if best < 0 { -1 } else { best % (10 ^ 9 + 7) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
