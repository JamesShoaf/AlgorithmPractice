/*
Given a positive integer n, generate an n x n matrix filled with elements from 1 to n2 in spiral
order.
*/

pub fn generate_spiral_matrix(n: i32) -> Vec<Vec<i32>> {
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![vec![1]];
    }
    let n = n as usize;
    let mut res = vec![vec![0; n]; n];
    let mut top = 0;
    let mut bottom = n - 1;
    let mut left = 0;
    let mut right = n - 1;
    let mut current = 1;
    while bottom >= top && right >= left {
        for col in left..=right {
            res[top][col] = current;
            current += 1;
        }
        top += 1;
        for row in top..=bottom {
            res[row][right] = current;
            current += 1;
        }
        right -= 1;
        for col in (left..=right).rev() {
            res[bottom][col] = current;
            current += 1;
        }
        bottom -= 1;
        for row in (top..=bottom).rev() {
            res[row][left] = current;
            current += 1;
        }
        left += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::generate_spiral_matrix;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            // (0, Vec::new()),
            (1, vec![vec![1]]),
            (2, vec![vec![1, 2], vec![4, 3]]),
            (3, vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]),
            (
                4,
                vec![
                    vec![1, 2, 3, 4],
                    vec![12, 13, 14, 5],
                    vec![11, 16, 15, 6],
                    vec![10, 9, 8, 7],
                ],
            ),
        ];
        for (n, expected) in test_tuples {
            assert_eq!(generate_spiral_matrix(n), expected);
        }
    }
}
