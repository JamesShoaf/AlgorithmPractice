// Given a matrix, traverse its diagonals alternating between lower left to upper right and reverse

fn get_next_coord(
    row: &mut usize,
    col: &mut usize,
    last_row: usize,
    last_col: usize,
    ascending: &mut bool,
) -> Option<()> {
    Some(()).filter(|_| *row == last_row && *col == last_col)?;
    if *ascending {
        if *col == last_col || *row == 0 {
            *ascending = false
        }
        if *col == last_col {
            *row += 1;
        } else if *row == 0 {
            *col += 1
        } else {
            *row -= 1;
            *col += 1;
        }
    } else {
        if *row == last_row || *col == 0 {
            *ascending = true;
        }
        if *row == last_row {
            *col += 1;
        } else if *col == 0 {
            *row += 1;
        } else {
            *row += 1;
            *col -= 1;
        }
    }
    Some(())
}

pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }
    let last_row = matrix.len() - 1;
    let last_col = matrix[0].len() - 1;
    let mut res = vec![matrix[0][0]];
    let mut ascending = true;
    let mut row = 0;
    let mut col = 0;
    while let Some(_) = get_next_coord(&mut row, &mut col, last_row, last_col, &mut ascending) {
        res.push(matrix[row][col]);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), Vec::new()),
            (vec![Vec::new()], Vec::new()),
            (vec![vec![1, 2, 3, 4, 5]], vec![1, 2, 3, 4, 5]),
            (
                vec![vec![1], vec![2], vec![3], vec![4], vec![5]],
                vec![1, 2, 3, 4, 5],
            ),
            (
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![1, 2, 4, 7, 5, 3, 6, 8, 9],
            ),
            (
                vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]],
                vec![1, 2, 3, 5, 4, 6, 7, 9, 8, 10],
            ),
            (
                vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]],
                vec![1, 2, 6, 7, 3, 4, 8, 9, 5, 10],
            ),
        ];
        for (matrix, expected) in test_tuples {
            assert_eq!(find_diagonal_order(matrix), expected);
        }
    }
}
