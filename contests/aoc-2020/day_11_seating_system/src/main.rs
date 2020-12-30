use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    if let Ok(f) = File::open("input.txt") {
        let f = BufReader::new(f);
        let matrix: Vec<Vec<char>> = f
            .lines()
            .filter_map(|line| line.ok())
            .map(|string| string.chars().collect())
            .collect();
        let stable = stable_seating(&matrix);
        let seated_count: usize = stable
            .iter()
            .map(|row| row.iter().filter(|&&char| char == '#').count())
            .sum();
        println!("seated: {}", seated_count);
    }
}

fn eight_neighbors(row: usize, col: usize, max_row: usize, max_col: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for i in if row == 0 { 0 } else { row - 1 }..=cmp::min(max_row, row + 1) {
        for j in if col == 0 { 0 } else { col - 1 }..=cmp::min(max_col, col + 1) {
            if i != row || j != col {
                res.push((i, j));
            }
        }
    }
    res
}

fn occupied_count(row: usize, col: usize, gen: usize, state: &Vec<Vec<Vec<char>>>) -> usize {
    eight_neighbors(row, col, state[gen].len() - 1, state[gen][0].len() - 1)
        .into_iter()
        .filter(|&(row, col)| state[(gen + 1) % 2][row][col] == '#')
        .count()
}

fn update_cell(row: usize, col: usize, gen: usize, state: &mut Vec<Vec<Vec<char>>>) {
    let prev = (gen + 1) % 2;
    state[gen][row][col] = match state[prev][row][col] {
        'L' if occupied_count(row, col, gen, state) == 0 => '#',
        '#' if occupied_count(row, col, gen, state) >= 4 => 'L',
        _ => state[prev][row][col],
    }
}

fn stable_seating(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return matrix.clone();
    }
    let mut state = Vec::new();
    state.push(vec![vec![' '; matrix[0].len()]; matrix.len()]);
    state.push(matrix.clone());
    let mut res = 0;
    loop {
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                update_cell(row, col, res % 2, &mut state);
            }
        }
        if state[0] == state[1] {
            break;
        }
        res += 1;
    }
    state[0].clone()
}
