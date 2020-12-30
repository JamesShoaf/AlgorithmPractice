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

enum Dir {
    L,
    D,
    R,
}

fn find_exit_col(dir: Dir, row: usize, col: usize, matrix: &Vec<Vec<i32>>) -> Option<usize> {
    if row == matrix.len() {
        return Some(col);
    }
    let slant = matrix[row][col];
    match dir {
        Dir::L => {
            return Some(()).filter(|_| slant == -1).and(find_exit_col(
                Dir::D,
                row + 1,
                col,
                matrix,
            ));
        }
        Dir::D => {
            if slant == 1 {
                return Some(())
                    .filter(|_| col + 1 < matrix[row].len())
                    .and(find_exit_col(Dir::R, row, col + 1, matrix));
            }
            return Some(())
                .filter(|_| col > 0)
                .and(find_exit_col(Dir::L, row, col - 1, matrix));
        }
        Dir::R => {
            return Some(()).filter(|_| slant == 1).and(find_exit_col(
                Dir::D,
                row + 1,
                col,
                matrix,
            ));
        }
    }
}

pub fn find_ball(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() {
        return Vec::new();
    }
    (0..matrix[0].len())
        .map(|i| {
            if let Some(col) = find_exit_col(Dir::D, 0, i, &matrix) {
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
