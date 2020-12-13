/*
Given n balloons, indexed from 0 to n-1. Each balloon is painted with a number on it represented by
array nums. You are asked to burst all the balloons. If the you burst balloon i you will get
nums[left] * nums[i] * nums[right] coins. Here left and right are adjacent indices of i. After the
burst, the left and right then becomes adjacent.

Find the maximum coins you can collect by bursting the balloons wisely.

Note:

    You may imagine nums[-1] = nums[n] = 1. They are not real therefore you can not burst them.
    0 ≤ n ≤ 500, 0 ≤ nums[i] ≤ 100

*/

// get the neighboring values to the subarray (or default to 1)
fn get_multipliers(l: usize, r: usize, nums: &Vec<i32>) -> (i32, i32) {
    let l_mult = if l == 0 { 1 } else { nums[l - 1] };
    let r_mult = if r == nums.len() - 1 { 1 } else { nums[r + 1] };
    (l_mult, r_mult)
}

// for each index k within a subarray, find the score for popping balloon k last assuming no
// balloons outside the subarray have been popped
fn get_max_value_for_subarray(l: usize, r: usize, dp: &Vec<Vec<i32>>, nums: &Vec<i32>) -> i32 {
    let (l_mult, r_mult) = get_multipliers(l, r, &nums);
    (l..=r)
        .map(|k| {
            let left_part = if k == l { 0 } else { dp[l][k - 1] };
            let right_part = if k == r { 0 } else { dp[k + 1][r] };
            let mid_part = l_mult * nums[k] * r_mult;
            left_part + right_part + mid_part
        })
        .max()
        .unwrap()
}

pub fn max_coins(nums: Vec<i32>) -> i32 {
    // optimization: pop all balloons with value 0 first
    let nums: Vec<i32> = nums.into_iter().filter(|&num| num > 0).collect();
    if nums.is_empty() {
        return 0;
    }

    // bottom-up dp - dp[l][r] represents the best score obtainable from a subarray l..=r
    let mut dp = vec![vec![0; nums.len()]; nums.len()];
    for len in 0..nums.len() {
        for l in 0..nums.len() - len {
            let r = l + len;
            dp[l][r] = get_max_value_for_subarray(l, r, &dp, &nums);
        }
    }
    *dp.first().unwrap().last().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
