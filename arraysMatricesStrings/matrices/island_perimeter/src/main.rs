use std::collections::HashSet;

struct Solution {}

impl Solution {
    fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let row_count = grid.len() as i32;
        if row_count == 0 { return 0 }
        let col_count = grid[0].len() as i32;
        if col_count == 0 { return 0 }
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut coord_stack: Vec<(i32, i32)> = Vec::new();
        let mut perimeter = 0;
        let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut edge_counter = |row: i32, col: i32, stack: &mut Vec<(i32, i32)>| {
            if visited.contains(&(row, col)) { return; } else { visited.insert((row, col)); }
            for (row_adj, col_adj) in neighbors.iter() {
                let new_row = row + row_adj;
                let new_col = col + col_adj;
                if !visited.contains(&(new_row, new_col)) {
                    if new_row < 0 || new_row == row_count
                    || new_col < 0 || new_col == col_count 
                    || grid[new_row as usize][new_col as usize] == 0 {
                        perimeter += 1;
                    } else { stack.push((new_row, new_col)) }
                }
            }
        };
        for row in 0..row_count {
            if coord_stack.len() > 0 { break; }
            for col in 0..col_count {
                if grid[row as usize][col as usize] == 1 {
                    coord_stack.push((row, col));
                    break;
                }
            }
        }
        while coord_stack.len() > 0 {
            let (next_row, next_col) = coord_stack.pop().unwrap();
            edge_counter(next_row, next_col, &mut coord_stack);
        }
        return perimeter;
    }
}

fn main() {
    let map = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    println!("{}", Solution::island_perimeter(map));
}
