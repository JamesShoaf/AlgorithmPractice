/*
You are given an integer array nums. You can choose exactly one index (0-indexed) and remove the
element. Notice that the index of the elements may change after the removal.

For example, if nums = [6,1,7,4,1]:

    Choosing to remove index 1 results in nums = [6,7,4,1].
    Choosing to remove index 2 results in nums = [6,1,4,1].
    Choosing to remove index 4 results in nums = [6,1,7,4].

An array is fair if the sum of the odd-indexed values equals the sum of the even-indexed values.

Return the number of indices that you could choose such that after the removal, nums is fair.
*/

pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
    let mut left_sum: Vec<Vec<i32>> = vec![vec![0; 2]; nums.len()];
    for i in 1..nums.len() {
        left_sum[i][0] = left_sum[i - 1][0];
        left_sum[i][1] = left_sum[i - 1][1];
        left_sum[i][(i + 1) % 2] += nums[i - 1];
    }
    let mut right_sum: Vec<Vec<i32>> = vec![vec![0; 2]; nums.len()];
    for i in (0..nums.len() - 1).rev() {
        right_sum[i][0] = right_sum[i + 1][0];
        right_sum[i][1] = right_sum[i + 1][1];
        right_sum[i][i % 2] += nums[i + 1];
    }
    (0..nums.len())
        .filter(|&i| left_sum[i][0] + right_sum[i][0] == left_sum[i][1] + right_sum[i][1])
        .count() as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
