// Given an n x n binary grid, in one step you can choose two adjacent rows of the grid and swap them.

// A grid is said to be valid if all the cells above the main diagonal are zeros.

// Return the minimum number of steps needed to make the grid valid, or -1 if the grid cannot be valid.

// The main diagonal of a grid is the diagonal that starts at cell (1, 1) and ends at cell (n, n).

struct Solution {}

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let dimensions = grid.len();
        let mut terminal_zero_count: Vec<i32> = Vec::new();
        let mut count_counts: Vec<i32> = vec![0; dimensions + 1];
        for row in grid {
            // println!("row: {:?}", row);
            let mut zero_count = 0;
            for i in 0..dimensions {
                if row[dimensions - i - 1] == 1 { break; }
                zero_count += 1;
            }
            // println!("zero_count: {}", zero_count);
            terminal_zero_count.push(zero_count);
            count_counts[zero_count as usize] += 1;
            // println!("counts: {:?}", count_counts);
            if count_counts[zero_count as usize] > zero_count + 1 { return -1;}
        }
        let mut shift_count = 0;
        // let mut test_counter = 0;
        loop {
            // println!("{:?}", terminal_zero_count);
            // test_counter += 1;
            let mut no_shifts = true;
            for i in 0..dimensions - 1 {
                if terminal_zero_count[i] < (dimensions - i - 1) as i32 {
                    let temp = terminal_zero_count[i];
                    terminal_zero_count[i] = terminal_zero_count[i + 1];
                    terminal_zero_count[i + 1] = temp;
                    shift_count += 1;
                    no_shifts = false;
                }
            }
            if no_shifts { break; }
            // if test_counter > dimensions * dimensions { break; }
        }
        shift_count
    }
}

fn main() {
    // let grid = vec![vec![0,0,1],vec![1,1,0],vec![1,0,0]];
    // Solution::min_swaps(grid);
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
