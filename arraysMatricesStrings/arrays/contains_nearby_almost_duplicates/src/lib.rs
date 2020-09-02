/*
Given an array of integers, find out whether there are two distinct indices i and j in the array
such that the absolute difference between nums[i] and nums[j] is at most t and the absolute
difference between i and j is at most k.
*/

use std::collections::BTreeSet;

fn contains_nearby_almost_duplicates(nums: Vec<i32>, k: i32, t: i32) -> bool {
    if k < 0 || t < 0 { return false; }
    let k = k as usize;
    let mut tree: BTreeSet<i32> = BTreeSet::new();
    for (i, num) in nums.iter().enumerate() {
        let num = *num;
        if i > k { tree.remove(&nums[i - (k + 1)]); }
        let min = if num < i32::MIN + t { i32::MIN } else { num - t };
        let max = if num > i32::MAX - t { i32:: MAX } else { num + t }; 
        for _ in tree.range((min)..=(max)) {
            return true;
        }
        tree.insert(num);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), 3, 0, false),
            (vec![1, 2, 3, 1], -3, 0, false),
            (vec![1, 2, 3, 1], 3, -1, false),
            (vec![1, 2, 3, 1], 10, 0, true),
            (vec![1, 2, 3, 1], 3, 0, true),
            (vec![1, 0, 1, 1], 1, 2, true),
            (vec![1, 5, 9, 1, 5, 9], 2, 3, false),
            (vec![0, i32::MAX], 1, i32::MAX, true),
        ];
        for (i, (nums, k, t, expected)) in test_tuples.iter().enumerate() {
            assert_eq!(contains_nearby_almost_duplicates(nums.clone(), *k, *t), *expected,
                "failed test {}", i);
        }
    }
}
