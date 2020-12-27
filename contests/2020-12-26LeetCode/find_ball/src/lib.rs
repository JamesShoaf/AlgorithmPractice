/*
You have a 2-D grid of size m x n representing a box, and you have n balls. The box is open on the
top and bottom sides.

Each cell in the box has a diagonal board spanning two corners of the cell that can redirect a ball
to the right or to the left.

    A board that redirects the ball to the right spans the top-left corner to the bottom-right
    corner and is represented in the grid as 1.
    A board that redirects the ball to the left spans the top-right corner to the bottom-left
    corner and is represented in the grid as -1.

We drop one ball at the top of each column of the box. Each ball can get stuck in the box or fall
out of the bottom. A ball gets stuck if it hits a "V" shaped pattern between two boards or if a
board redirects the ball into either wall of the box.

Return an array answer of size n where answer[i] is the column that the ball falls out of at the
bottom after dropping the ball from the ith column at the top, or -1 if the ball gets stuck in the box.
*/

enum Direction {
    Left,
    Down,
    Right,
}

fn find_exit_col(
    direction: Direction,
    row: usize,
    col: usize,
    matrix: &Vec<Vec<i32>>,
) -> Option<usize> {
    if row == matrix.len() {
        return Some(col);
    }
    let slant = matrix[row][col];
    match direction {
        Direction::Left => {
            if slant == 1 {
                return None;
            }
            return find_exit_col(Direction::Down, row + 1, col, matrix);
        }
        Direction::Down => {
            if slant == 1 {
                if col + 1 == matrix[row].len() {
                    return None;
                }
                return find_exit_col(Direction::Right, row, col + 1, matrix);
            }
            if col == 0 {
                return None;
            }
            return find_exit_col(Direction::Left, row, col - 1, matrix);
        }
        Direction::Right => {
            if slant == -1 {
                return None;
            }
            return find_exit_col(Direction::Down, row + 1, col, matrix);
        }
    }
}

pub fn find_ball(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() {
        return Vec::new();
    }
    (0..matrix[0].len())
        .map(|i| {
            if let Some(col) = find_exit_col(Direction::Down, 0, i, &matrix) {
                return col as i32;
            }
            return -1;
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
