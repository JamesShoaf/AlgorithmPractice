/*
You are given a 0-indexed integer array nums and an integer k.

You are initially standing at index 0. In one move, you can jump at most k steps forward without
going outside the boundaries of the array. That is, you can jump from index i to any index in the
range [i + 1, min(n - 1, i + k)] inclusive.

You want to reach the last index of the array (index n - 1). Your score is the sum of all nums[j]
for each index j you visited in the array.

Return the maximum score you can get.
*/

use std::cmp;

pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut dp = vec![*nums.last().unwrap(); nums.len()];
    for i in (0..nums.len() - 1).rev() {
        let mut best = i32::MIN;
        for j in i + 1..=cmp::min(nums.len() - 1, i + k) {
            best = cmp::max(best, nums[i] + dp[j]);
            if nums[j] >= 0 {
                break;
            }
        }
        dp[i] = best;
    }
    *dp.first().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
