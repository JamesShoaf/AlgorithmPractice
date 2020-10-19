/* 
In a row of dominoes, A[i] and B[i] represent the top and bottom halves of the ith domino.  (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)

We may rotate the ith domino, so that A[i] and B[i] swap values.

Return the minimum number of rotations so that all the values in A are the same, or all the values in B are the same.

If it cannot be done, return -1.
*/

use std::cmp;

fn count_rotations(a: &Vec<i32>, b: &Vec<i32>) -> Option<i32> {
    // if all dominoes can be flipped to a matching value
    if (1..a.len()).all(|i| a[i] == a[0] || b[i] == a[0]) {
        // count number of flips to get top row to match first element of top row
        return Some(cmp::min(a[1..].iter().filter(|&&val| val != a[0]).count(),
        // then compare with flipping dominoes the opposite way to match the value in the bottom row
            1 + b[1..].iter().filter(|&&val| val != a[0]).count()) as i32);
    }
    None
}

pub fn min_domino_rotations(a: Vec<i32>, b: Vec<i32>) -> i32 {
    if a.len() < 2 { return 0; }
    if let Some(a_val) = count_rotations(&a, &b) {
        if let Some(b_val) = count_rotations(&b, &a) {
            return cmp::min(a_val, b_val);
        }
        return a_val;
    }
    if let Some(b_val) = count_rotations(&b, &a) {
        return b_val;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(),
            Vec::new(),
            0),
            (vec![1],
            vec![3],
            0),
            (vec![2, 1, 2, 4, 2, 2],
            vec![5, 2, 6, 2, 3, 2],
            2),
            (vec![3, 5, 1, 2, 3],
            vec![3, 6, 3, 3, 4],
            -1),
            (vec![1, 2, 3],
            vec![3, 3, 3],
            0),
        ];
        for (a, b, expected) in test_tuples {
            assert_eq!(min_domino_rotations(a, b), expected);
        }
    }
}
