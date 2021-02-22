/*
You are given two integer arrays nums and multipliers of size n and m respectively, where n >= m.
The arrays are 1-indexed.

You begin with a score of 0. You want to perform exactly m operations. On the ith operation
(1-indexed), you will:

    Choose one integer x from either the start or the end of the array nums.
    Add multipliers[i] * x to your score.
    Remove x from the array nums.

Return the maximum score after performing m operations.
*/

use std::cmp;
use std::collections::HashMap;

fn dfs(
    nums_l: usize,
    mult_index: usize,
    nums: &Vec<i32>,
    mult: &Vec<i32>,
    memo: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if mult_index == mult.len() {
        return 0;
    }
    if let Some(&prev) = memo.get(&(nums_l, mult_index)) {
        return prev;
    }
    let mut res =
        mult[mult_index] * nums[nums_l] + dfs(nums_l + 1, mult_index + 1, nums, mult, memo);
    let nums_r = nums.len() + nums_l - mult_index - 1;
    if nums_r > nums_l {
        res = cmp::max(
            res,
            mult[mult_index] * nums[nums_r] + dfs(nums_l, mult_index + 1, nums, mult, memo),
        );
    }
    memo.insert((nums_l, mult_index), res);
    res
}

pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
    let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
    dfs(0, 0, &nums, &multipliers, &mut memo)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
