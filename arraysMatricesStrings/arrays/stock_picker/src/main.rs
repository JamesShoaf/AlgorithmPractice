/* 
Say you have an array for which the ith element is the price of a given stock on day i.

Design an algorithm to find the maximum profit. You may complete at most two transactions.
*/

fn max_profit(prices: Vec<i32>) -> i32 {
    use std::cmp::{max, min};
    let mut b1 = i32::MAX;
    let mut s1 = 0;
    let mut b2 = i32::MAX;
    let mut s2 = 0;
    for p in prices {
        b1 = min(b1, p);
        s1 = max(s1, p - b1);
        b2 = min(b2, p - s1);
        s2 = max(s2, p - b2);
    }
    s2
}

fn main() {
    let test_tuples = vec![
        (vec![3, 3, 5, 0, 0, 3, 1, 4], 6),
        (vec![0, 1, 0, 2, 0, 3], 5),
        (vec![0, 3, 0, 2, 0, 1], 5),
        (vec![0, 3, 0, 1, 0, 2], 5),
        (vec![0, 1, 0, 3, 0, 2], 5),
        (vec![0, 2, 0, 1, 0, 3], 5),
        (vec![0, 2, 0, 3, 0, 1], 5),
        (vec![1, 2, 3, 4, 5], 4),
        (vec![7, 6, 4, 3, 1], 0),
    ];
    for (nums, expected) in test_tuples { assert_eq!(max_profit(nums), expected); }
}
