/*
A game is played by a cat and a mouse named Cat and Mouse.

The environment is represented by a grid of size rows x cols, where each element is a wall, floor, player (Cat, Mouse), or food.

    Players are represented by the characters 'C'(Cat),'M'(Mouse).
    Floors are represented by the character '.' and can be walked on.
    Walls are represented by the character '#' and cannot be walked on.
    Food is represented by the character 'F' and can be walked on.
    There is only one of each character 'C', 'M', and 'F' in grid.

Mouse and Cat play according to the following rules:

    Mouse moves first, then they take turns to move.
    During each turn, Cat and Mouse can jump in one of the four directions (left, right, up, down). They cannot jump over the wall nor outside of the grid.
    catJump, mouseJump are the maximum lengths Cat and Mouse can jump at a time, respectively. Cat and Mouse can jump less than the maximum length.
    Staying in the same position is allowed.
    Mouse can jump over Cat.

The game can end in 4 ways:

    If Cat occupies the same position as Mouse, Cat wins.
    If Cat reaches the food first, Cat wins.
    If Mouse reaches the food first, Mouse wins.
    If Mouse cannot get to the food within 1000 turns, Cat wins.

Given a rows x cols matrix grid and two integers catJump and mouseJump, return true if Mouse can win the game if both Cat and Mouse play optimally, otherwise return false.
*/

use std::collections::HashMap;
type Memo = HashMap<(u32, (usize, usize), (usize, usize)), bool>;

pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
    let mut cat = (0, 0);
    let mut mouse = (0, 0);
    let mut board: Vec<Vec<char>> = vec![vec!['.'; grid[0].len()]; grid.len()];
    for i in 0..grid.len() {
        for (j, c) in grid[i].chars().enumerate() {
            match c {
                '#' | 'F' => board[i][j] = c,
                'C' => cat = (i, j),
                'M' => mouse = (i, j),
                _ => (),
            }
        }
    }
    let mut memo: Memo = HashMap::new();
    dp(
        0,
        cat,
        mouse,
        cat_jump as usize,
        mouse_jump as usize,
        &board,
        &mut memo,
    )
}

// Given an 8x8 board, the cat and mouse can explore every possible square within 128 turns
const TURN_LIMIT: u32 = 128;
use std::cmp;
fn dp(
    turn: u32,
    cat: (usize, usize),
    mouse: (usize, usize),
    cat_jump: usize,
    mouse_jump: usize,
    board: &Vec<Vec<char>>,
    memo: &mut Memo,
) -> bool {
    // memoized dfs for moves that will cause the other party to lose
    // Whoever reaches the food first wins. The cat also wins if the turn limit expires
    if board[cat.0][cat.1] == 'F' || board[mouse.0][mouse.1] == 'F' || turn == TURN_LIMIT {
        return false;
    }
    // The cat wins if it catches the mouse
    if cat == mouse {
        return turn % 2 != 0;
    }
    if let Some(&prev) = memo.get(&(turn, cat, mouse)) {
        return prev;
    }
    let mut res = !dp(turn + 1, cat, mouse, cat_jump, mouse_jump, board, memo);
    // rewrite to handle walls
    // if turn % 2 == 0 {
    //     let mut x = if mouse.0 > mouse_jump {
    //         mouse.0 - mouse_jump
    //     } else {
    //         0
    //     };
    //     let high_x = cmp::min(board.len(), mouse.0 + mouse_jump + 1);
    //     while !res && x < high_x {
    //         if board[x][mouse.1] != '#' {
    //             res = dp(
    //                 turn + 1,
    //                 cat,
    //                 (x, mouse.1),
    //                 cat_jump,
    //                 mouse_jump,
    //                 board,
    //                 memo,
    //             );
    //         }
    //         x += 1;
    //     }
    //     let mut y = if mouse.1 > mouse_jump {
    //         mouse.1 - mouse_jump
    //     } else {
    //         0
    //     };
    //     let high_y = cmp::min(board.len(), mouse.1 + mouse_jump + 1);
    //     while !res && y < high_y {
    //         if board[mouse.0][y] != '#' {
    //             res = dp(
    //                 turn + 1,
    //                 cat,
    //                 (mouse.0, y),
    //                 cat_jump,
    //                 mouse_jump,
    //                 board,
    //                 memo,
    //             );
    //         }
    //         y += 1;
    //     }
    // } else {
    //     let mut x = if cat.0 > cat_jump {
    //         cat.0 - cat_jump
    //     } else {
    //         0
    //     };
    //     let high_x = cmp::min(board.len(), cat.0 + cat_jump + 1);
    //     while !res && x < high_x {
    //         if board[x][cat.1] != '#' {
    //             res = dp(
    //                 turn + 1,
    //                 (x, cat.1),
    //                 mouse,
    //                 cat_jump,
    //                 mouse_jump,
    //                 board,
    //                 memo,
    //             );
    //         }
    //         x += 1;
    //     }
    //     let mut y = if cat.1 > cat_jump {
    //         cat.1 - cat_jump
    //     } else {
    //         0
    //     };
    //     let high_y = cmp::min(board.len(), cat.1 + cat_jump + 1);
    //     while !res && y < high_y {
    //         if board[cat.0][y] != '#' {
    //             res = dp(turn + 1, (cat.0, y), cat, cat_jump, mouse_jump, board, memo);
    //         }
    //         y += 1;
    //     }
    // }
    memo.insert((turn, cat, mouse), res);
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
