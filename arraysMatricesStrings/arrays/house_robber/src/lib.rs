/* 
You are a professional robber planning to rob houses along a street. Each house has a certain
amount of money stashed, the only constraint stopping you from robbing each of them is that
adjacent houses have security system connected and it will automatically contact the police if two
adjacent houses were broken into on the same night.

Given a list of non-negative integers representing the amount of money of each house, determine the
maximum amount of money you can rob tonight without alerting the police.
*/

use std::cmp;

fn rob(marks: Vec<i32>) -> i32 {
    let mut memo: Vec<i32> = vec![0, 0];
    for i in (0..marks.len()).rev() {
        memo[i % 2] = cmp::max(memo[(i + 1) % 2], marks[i] + memo[i % 2]);
    }
    memo[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let cased_houses = vec![
            (Vec::new(), 0),
            (vec![1], 1),
            (vec![1, 1], 1),
            (vec![2, 1], 2),
            (vec![1, 2], 2),
            (vec![1, 2, 3, 1], 4),
            (vec![2, 7, 9, 3, 1], 12),
            (vec![2, 7, 2, 9, 3, 1], 17),
            (vec![0, 0, 0, 0, 2, 3, 0, 0], 3),
        ];

        for (marks, expected) in cased_houses {
            assert_eq!(rob(marks), expected);
        }
    }
}
