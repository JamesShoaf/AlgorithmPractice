/*
You are given an integer matrix isWater of size m x n that represents a map of land and water cells.

    If isWater[i][j] == 0, cell (i, j) is a land cell.
    If isWater[i][j] == 1, cell (i, j) is a water cell.

You must assign each cell a height in a way that follows these rules:

    The height of each cell must be non-negative.
    If the cell is a water cell, its height must be 0.
    Any two adjacent cells must have an absolute height difference of at most 1. A cell is adjacent
    to another cell if the former is directly north, east, south, or west of the latter (i.e.,
        their sides are touching).

Find an assignment of heights such that the maximum height in the matrix is maximized.

Return an integer matrix height of size m x n where height[i][j] is cell (i, j)'s height. If there
are multiple solutions, return any of them.
*/

use std::collections::HashSet;

pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if is_water.is_empty() {
        return Vec::new();
    }
    let mut res = vec![vec![-1; is_water[0].len()]; is_water.len()];
    let mut coord_set: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..is_water.len() {
        for j in 0..is_water[0].len() {
            if is_water[i][j] == 1 {
                coord_set.insert((i, j));
            }
        }
    }
    let mut height = 0;
    while !coord_set.is_empty() {
        let mut next_set: HashSet<(usize, usize)> = HashSet::new();
        for (i, j) in coord_set {
            if res[i][j] == -1 {
                res[i][j] = height;
                if i > 0 && res[i - 1][j] == -1 {
                    next_set.insert((i - 1, j));
                }
                if i + 1 < res.len() && res[i + 1][j] == -1 {
                    next_set.insert((i + 1, j));
                }
                if j > 0 && res[i][j - 1] == -1 {
                    next_set.insert((i, j - 1));
                }
                if j + 1 < res[0].len() && res[i][j + 1] == -1 {
                    next_set.insert((i, j + 1));
                }
            }
        }
        height += 1;
        coord_set = next_set;
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
