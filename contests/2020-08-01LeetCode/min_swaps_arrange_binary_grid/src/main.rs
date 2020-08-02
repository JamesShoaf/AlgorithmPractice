// Given an n x n binary grid, in one step you can choose two adjacent rows of the grid and swap them.

// A grid is said to be valid if all the cells above the main diagonal are zeros.

// Return the minimum number of steps needed to make the grid valid, or -1 if the grid cannot be valid.

// The main diagonal of a grid is the diagonal that starts at cell (1, 1) and ends at cell (n, n).

struct Solution {}

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let len = grid.len();
        let mut zero_count: Vec<i32> = Vec::new();
        for row in grid {
            let mut row_count = 0;
            for i in 0..len {
                if row[len - i - 1] == 1 { break; }
                row_count += 1;
            }
            zero_count.push(row_count);
        }
        let mut shift_count = 0;
        for i in 0..len {
            let target = (len - i - 1) as i32;
            if zero_count[i] < target {
                let j = {
                    let mut index = len;
                    for j in i + 1..len {
                        if zero_count[j] >= target {
                            index = j;
                            break;
                        }
                    }
                    if index == len { return -1; }
                    index
                };
                for k in (i..j).rev() {
                    let temp = zero_count[k];
                    zero_count[k] = zero_count[k + 1];
                    zero_count[k + 1] = temp;
                    shift_count += 1;
                }
            }
        }
        shift_count
    }
}

fn main() {
    let test_tuples = vec![
        (vec![vec![0,0,1],vec![1,1,0],vec![1,0,0]], 3),
        (vec![vec![0,1,1,0],vec![0,1,1,0],vec![0,1,1,0],vec![0,1,1,0]], -1),
        (vec![vec![1,0,0],vec![1,1,0],vec![1,1,1]], 0),
        (vec![vec![1,0,0,0,0,0],vec![0,0,0,1,0,0],vec![0,0,0,1,0,0],vec![0,1,0,0,0,0],vec![0,0,1,0,0,0],vec![0,0,0,0,0,1]], 4),
    ];
    for (grid, expected) in test_tuples {
        assert_eq!(Solution::min_swaps(grid), expected);
    }
}
