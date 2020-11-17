/*
Let's call any (contiguous) subarray B (of A) a mountain if the following properties hold:

    B.length >= 3
    There exists some 0 < i < B.length - 1 such that B[0] < B[1] < ... B[i-1] < B[i] > B[i+1] > ... > B[B.length - 1]

(Note that B could be any subarray of A, including the entire array A.)

Given an array A of integers, return the length of the longest mountain.

Return 0 if there is no mountain.
*/

use std::cmp::{self, Ordering};

pub fn longest_mountain(a: Vec<i32>) -> i32 {
    if a.is_empty() {
        return 0;
    }
    let mut start = 0;
    let mut peak = 0;
    let mut res = 0;
    for i in 1..a.len() {
        match a[i].cmp(&a[i - 1]) {
            Ordering::Equal => {
                start = i;
                peak = i;
            }
            Ordering::Greater => {
                peak = i;
                if i >= 2 && a[i - 1] < a[i - 2] {
                    start = i - 1;
                }
            }
            Ordering::Less if start == peak => {
                start = i;
                peak = i;
            }
            Ordering::Less if i - start > 1 => res = cmp::max(res, i - start + 1),
            _ => (),
        }
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![2, 1, 4, 7, 3, 2, 5], 5),
            (vec![0, 1], 0),
            (vec![1, 0], 0),
            (vec![2, 2, 2], 0),
            (vec![1, 2, 3], 0),
            (vec![3, 2, 1], 0),
            (vec![1, 2, 1], 3),
            (vec![1, 2, 3, 1], 4),
            (vec![1, 3, 2, 1], 4),
            (vec![1, 3, 2, -1, 0, -1], 4),
            (vec![-1, 0, -1, 0, 0, -1], 3),
            (vec![1, 2, 3, 2, 3, 2, 1], 4),
        ];

        for (a, expected) in test_tuples {
            assert_eq!(longest_mountain(a.clone()), expected, "{:?}", a);
        }
    }
}
