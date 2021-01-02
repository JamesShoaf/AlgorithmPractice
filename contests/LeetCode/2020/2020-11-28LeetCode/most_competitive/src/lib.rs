/*
Given an integer array nums and a positive integer k, return the most competitive subsequence of
nums of size k.

An array's subsequence is a resulting sequence obtained by erasing some (possibly zero) elements
from the array.

We define that a subsequence a is more competitive than a subsequence b (of the same length) if
in the first position where a and b differ, subsequence a has a number less than the corresponding
number in b. For example, [1,3,4] is more competitive than [1,3,5] because the first position
they differ is at the final number, and 4 is less than 5.
*/

pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    if nums.len() == k {
        return nums;
    }
    let mut res: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        loop {
            if !res.is_empty() && nums[i] < *res.last().unwrap() && nums.len() - i > k - res.len() {
                res.pop();
            } else {
                if res.len() < k {
                    res.push(nums[i]);
                }
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![3, 5, 2, 6], 2, vec![2, 6]),
            (vec![2, 4, 3, 3, 5, 4, 9, 6], 4, vec![2, 3, 3, 4]),
            (vec![2, 4, 3, 3, 5, 4, 1, 9], 4, vec![2, 3, 1, 9]),
        ];
        for (nums, k, expected) in test_tuples {
            assert_eq!(most_competitive(nums, k), expected);
        }
    }
}
