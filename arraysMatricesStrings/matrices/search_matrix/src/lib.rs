/* 
Given a sorted m x n matrix, return whether a value is contained in the matrix.
The matrix is sorted such that
    each row is sorted in ascending order
    the first integer of each row is greater than the last integer of the previous row
*/

use std::cmp::Ordering;

pub fn search_matrix(matrix: &Vec<Vec<i32>>, val: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() { return false; }
    let mut low = 0;
    let mut high = matrix.len();
    while low < high {
        let mid = (low + high) / 2;
        match val.cmp(&matrix[mid][0]) {
            Ordering::Less => high = mid,
            Ordering::Equal => return true,
            Ordering::Greater => {
                if low == mid { break; }
                low = mid;
            }
        }
    }
    let row = low;
    low = 0;
    high = matrix[row].len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        match val.cmp(&matrix[row][mid]) {
            Ordering::Less => {
                if mid == 0 { return false; }
                high = mid - 1;
            }
            Ordering::Equal => return true,
            Ordering::Greater => low = mid + 1,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search_matrix() {
        assert_eq!(search_matrix(&Vec::new(), 0), false);
        assert_eq!(search_matrix(&vec![Vec::new()], 0), false);
        let matrix1 = vec![
            vec![1],
        ];
        let matrix2 = vec![
            vec![1, 3],
            vec![5, 7],
        ];
        let matrix3 = vec![
            vec![1, 3, 5],
            vec![7, 9, 11],
            vec![13, 15, 17],
        ];
        let matrix4 = vec![
            vec![1, 3, 5, 7],
            vec![9, 11, 13, 15],
            vec![17, 19, 21, 23],
            vec![25, 27, 29, 31],
        ];
        let test_tuples = vec![
            (&matrix1, 3),
            (&matrix2, 9),
            (&matrix3, 19),
            (&matrix4, 33),
        ];
        for (matrix, upper) in test_tuples {
            for i in 0..upper {
                assert_eq!(search_matrix(matrix, i), i % 2 == 1);
            }
        }
    }
}
