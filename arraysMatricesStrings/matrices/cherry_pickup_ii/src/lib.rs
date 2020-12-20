/*
Given a rows x cols matrix grid representing a field of cherries. Each cell in grid represents the
number of cherries that you can collect.

You have two robots that can collect cherries for you, Robot #1 is located at the top-left corner
(0,0) , and Robot #2 is located at the top-right corner (0, cols-1) of the grid.

Return the maximum number of cherries collection using both robots  by following the rules below:

    From a cell (i,j), robots can move to cell (i+1, j-1) , (i+1, j) or (i+1, j+1).
    When any robot is passing through a cell, It picks it up all cherries, and the cell becomes an
        empty cell (0).
    When both robots stay on the same cell, only one of them takes the cherries.
    Both robots cannot move outside of the grid at any moment.
    Both robots should reach the bottom row in the grid.

*/

use std::cmp;
use std::collections::HashMap;

fn get_next_cols(col: usize, col_count: usize) -> Vec<usize> {
    let mut cols = Vec::new();
    if col > 0 {
        cols.push(col - 1);
    }
    cols.push(col);
    if col < col_count - 1 {
        cols.push(col + 1);
    }
    cols
}

// given the robots' two starting columns, return all possible next column positions
fn get_next_col_pairs(col_1: usize, col_2: usize, col_count: usize) -> Vec<(usize, usize)> {
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let next_col_2 = get_next_cols(col_2, col_count);
    for i in get_next_cols(col_1, col_count) {
        for &j in &next_col_2 {
            if i <= j {
                pairs.push((i, j));
            }
        }
    }
    pairs
}

fn recursive_helper(
    row: usize,
    col_1: usize,
    col_2: usize,
    grid: &Vec<Vec<i32>>,
    memo: &mut HashMap<(usize, usize, usize), i32>,
) -> i32 {
    if row == grid.len() {
        return 0;
    }
    if let Some(&prev) = memo.get(&(row, col_1, col_2)) {
        return prev;
    }
    let score = grid[row][col_1] + if col_2 != col_1 { grid[row][col_2] } else { 0 };
    let mut max = 0;
    for (i, j) in get_next_col_pairs(col_1, col_2, grid[row].len()) {
        max = cmp::max(max, recursive_helper(row + 1, i, j, grid, memo));
    }
    memo.insert((row, col_1, col_2), max + score);
    max + score
}

pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }
    let col_count = grid[0].len();
    let mut memo: HashMap<(usize, usize, usize), i32> = HashMap::new();
    recursive_helper(0, 0, col_count - 1, &grid, &mut memo)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
