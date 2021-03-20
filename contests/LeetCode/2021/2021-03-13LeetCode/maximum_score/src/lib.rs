/*
You are given an array of integers nums (0-indexed) and an integer k.

The score of a subarray (i, j) is defined as
min(nums[i], nums[i+1], ..., nums[j]) * (j - i + 1). A good subarray is a subarray where i <= k <= j.

Return the maximum possible score of a good subarray.
*/

use std::cmp;

pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut score = nums[k];
    let mut min = nums[k];
    let mut i = k;
    let mut j = k;
    while i > 0 || j < nums.len() - 1 {
        if i == 0 || j < nums.len() - 1 && nums[j + 1] >= nums[i - 1] {
            j += 1;
            min = cmp::min(min, nums[j]);
        } else {
            i -= 1;
            min = cmp::min(min, nums[i]);
        }
        score = cmp::max(score, min * (j - i + 1) as i32);
    }
    score
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
