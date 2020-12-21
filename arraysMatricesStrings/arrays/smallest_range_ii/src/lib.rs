/*
Given an array A of integers, for each integer A[i] we need to choose either x = -K or x = K, and
add x to A[i] (only once).

After this process, we have some array B.

Return the smallest possible difference between the maximum value of B and the minimum value of B.
*/

use std::cmp;

// Assume there exists some midpoint below which numbers will be incremented and
// above which numbers will be decremented. Find the range based on that midpoint
pub fn smallest_range_ii(mut a: Vec<i32>, k: i32) -> i32 {
    if a.is_empty() {
        return 0;
    }
    a.sort();
    // If the midpoint exists outside the array, all numbers are increased/decreased - the range is unchanged
    let mut res = a.last().unwrap() - a.first().unwrap();
    // If the midpoint is within the range, the leftmost index will be incremented
    let left = a.first().unwrap() + k;
    // and the rightmost will be decremented
    let right = a.last().unwrap() - k;
    // test each pair of integers to see if the best midpoint lies between them
    for i in 1..a.len() {
        // incremented lower vs decremented highest
        let max = cmp::max(right, a[i - 1] + k);
        // decremented higher vs incremented lowest
        let min = cmp::min(left, a[i] - k);
        // save any range improvements
        res = cmp::min(res, max - min)
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![(vec![1], 0, 0), (vec![0, 10], 2, 6), (vec![1, 3, 6], 3, 3)];
        for (a, k, expected) in test_tuples {
            assert_eq!(smallest_range_ii(a, k), expected);
        }
    }
}
