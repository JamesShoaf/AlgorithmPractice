/*
Given an array nums of distinct positive integers, return the number of tuples (a, b, c, d)
such that a * b = c * d where a, b, c, and d are elements of nums, and a != b != c != d.
*/

use std::cmp;
use std::collections::{HashMap, HashSet};

pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
    let mut products: HashMap<i32, HashSet<i32>> = HashMap::new();
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] != nums[j] {
                products
                    .entry(nums[i] * nums[j])
                    .or_insert(HashSet::new())
                    .insert(cmp::min(nums[i], nums[j]));
            }
        }
    }
    products
        .values()
        .map(|set| 4 * (set.len() * (set.len() - 1)))
        .sum::<usize>() as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
