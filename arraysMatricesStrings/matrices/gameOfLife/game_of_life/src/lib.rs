/*
ccording to Wikipedia's article: "The Game of Life, also known simply as Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970."

The board is made up of an m x n grid of cells, where each cell has an initial state: live (represented by a 1) or dead (represented by a 0). Each cell interacts with its eight neighbors (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):

    Any live cell with fewer than two live neighbors dies as if caused by under-population.
    Any live cell with two or three live neighbors lives on to the next generation.
    Any live cell with more than three live neighbors dies, as if by over-population.
    Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

The next state is created by applying the above rules simultaneously to every cell in the current state, where births and deaths occur simultaneously. Given the current state of the m x n grid board, return the next state.
*/

use std::cmp;

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

// in place size-limited 1 generation state progression
pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    if !board.is_empty() && !board[0].is_empty() {
        let max_row = board.len() - 1;
        let max_col = board[0].len() - 1;
        for row in 0..board.len() {
            for col in 0..board[0].len() {
                let live_neighbors = eight_neighbors(row, col, max_row, max_col)
                    .into_iter()
                    .filter(|&(row, col)| board[row][col] % 2 == 1)
                    .count();
                board[row][col] = match board[row][col] {
                    0 if live_neighbors == 3 => 2,
                    1 if live_neighbors < 2 || live_neighbors > 3 => 3,
                    _ => board[row][col],
                }
            }
        }
        for row in 0..board.len() {
            for col in 0..board[0].len() {
                board[row][col] = match board[row][col] {
                    2 => 1,
                    3 => 0,
                    _ => board[row][col],
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
