/*
You are given an array nums that consists of non-negative integers. Let us define rev(x) as the reverse of the non-negative integer x. For example, rev(123) = 321, and rev(120) = 21. A pair of indices (i, j) is nice if it satisfies all of the following conditions:

    0 <= i < j < nums.length
    nums[i] + rev(nums[j]) == nums[j] + rev(nums[i])

Return the number of nice pairs of indices. Since that number can be too large, return it modulo 109 + 7.
*/

use std::collections::HashMap;

fn reverse(mut n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut res = 0;
    while n % 10 == 0 {
        n /= 10;
    }
    while n != 0 {
        res *= 10;
        res += n % 10;
        n /= 10;
    }
    res
}

pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut sum_map: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        let sum = nums[i] - reverse(nums[i]);
        *sum_map.entry(sum).or_insert(0) += 1;
    }
    for i in 0..nums.len() {
        let sum = nums[i] - reverse(nums[i]);
        let count = sum_map.entry(sum).or_insert(0);
        *count -= 1;
        res += *count;
    }
    res as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
