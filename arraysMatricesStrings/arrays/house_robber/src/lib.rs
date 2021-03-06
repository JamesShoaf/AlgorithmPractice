/* 
You are a professional robber planning to rob houses along a street. Each house has a certain
amount of money stashed, the only constraint stopping you from robbing each of them is that
adjacent houses have security system connected and it will automatically contact the police if two
adjacent houses were broken into on the same night.

Given a list of non-negative integers representing the amount of money of each house, determine the
maximum amount of money you can rob tonight without alerting the police.
*/

use std::cmp;

pub fn rob(marks: Vec<i32>) -> i32 {
    let mut memo: Vec<i32> = vec![0, 0];
    for i in (0..marks.len()).rev() {
        memo[i % 2] = cmp::max(memo[(i + 1) % 2], marks[i] + memo[i % 2]);
    }
    memo[0]
}

/* 
You are a professional robber planning to rob houses along a street. Each house has a certain
amount of money stashed. All houses at this place are arranged in a circle. That means the first
house is the neighbor of the last one. Meanwhile, adjacent houses have a security system connected,
and it will automatically contact the police if two adjacent houses were broken into on the same
night.

Given a list of non-negative integers nums representing the amount of money of each house, return
the maximum amount of money you can rob tonight without alerting the police.
*/

pub fn rob_ii(marks: Vec<i32>) -> i32 {
    match marks.len() {
        0 => 0,
        1 => marks[0],
        _ => cmp::max(rob(marks[..marks.len() - 1].to_vec()), rob(marks[1..].to_vec())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rob() {
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
            (vec![2, 3, 2], 4),
        ];

        for (marks, expected) in cased_houses {
            assert_eq!(rob(marks), expected);
        }
    }

    #[test]
    fn test_rob_ii() {
        let cased_houses = vec![
            (Vec::new(), 0),
            (vec![1], 1),
            (vec![1, 1], 1),
            (vec![2, 1], 2),
            (vec![1, 2], 2),
            (vec![1, 2, 3, 1], 4),
            (vec![2, 7, 9, 3, 1], 11),
            (vec![2, 7, 2, 9, 3, 1], 17),
            (vec![0, 0, 0, 0, 2, 3, 0, 0], 3),
            (vec![2, 3, 2], 3),
        ];

        for (marks, expected) in cased_houses {
            assert_eq!(rob_ii(marks), expected);
        }
    }
}
