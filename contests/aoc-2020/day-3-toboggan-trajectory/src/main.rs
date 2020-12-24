use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in f.lines() {
        if let Ok(line) = line {
            matrix.push(line.chars().collect());
        }
    }
    let count = toboggan_trajectory(3, 1, &matrix);
    println!("Trees on trajectory Right: 3, Down: 1: {}", count);
    let trajectories = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product_count = trajectories.into_iter().fold(1, |acc, (right, down)| {
        acc * toboggan_trajectory(right, down, &matrix)
    });
    println!("Product count: {}", product_count);
}

// given a trajectory vectory <right, down> and a starting position (0, 0), how many trees (#)
// will you encounter before reaching the bottom of the slope (). For reasons, the pattern of
// trees repeats left to right, but not up to down.
fn toboggan_trajectory(right: usize, down: usize, matrix: &Vec<Vec<char>>) -> usize {
    let mut col = 0;
    let width = matrix[0].len();
    let mut counter = 0;
    for row in (0..matrix.len()).step_by(down) {
        if matrix[row][col] == '#' {
            counter += 1;
        }
        col += right * down;
        col %= width
    }
    counter
}
