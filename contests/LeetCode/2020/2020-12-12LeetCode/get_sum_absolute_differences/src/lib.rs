/*
You are given an integer array nums sorted in non-decreasing order.

Build and return an integer array result with the same length as nums such that result[i] is equal
to the summation of absolute differences between nums[i] and all the other elements in the array.

In other words, result[i] is equal to sum(|nums[i]-nums[j]|) where 0 <= j < nums.length and
j != i (0-indexed).
*/

pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
    let mut left_part = vec![0; nums.len()];
    for i in 1..nums.len() {
        left_part[i] = left_part[i - 1] + (nums[i] - nums[i - 1]) * i as i32;
    }
    let mut right_part = vec![0; nums.len()];
    for i in (0..nums.len() - 1).rev() {
        right_part[i] = right_part[i + 1] + (nums[i + 1] - nums[i]) * (nums.len() - i - 1) as i32;
    }
    left_part
        .into_iter()
        .zip(right_part.into_iter())
        .map(|(left, right)| left + right)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
