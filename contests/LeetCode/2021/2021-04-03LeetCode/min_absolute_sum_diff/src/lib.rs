/*
You are given two positive integer arrays nums1 and nums2, both of length n.

The absolute sum difference of arrays nums1 and nums2 is defined as the sum of |nums1[i] - nums2[i]| for each 0 <= i < n (0-indexed).

You can replace at most one element of nums1 with any other element in nums1 to minimize the absolute sum difference.

Return the minimum absolute sum difference after replacing at most one element in the array nums1. Since the answer may be large, return it modulo 109 + 7.
*/

use std::cmp;

// binary search sorted for the closest value to k in sorted array
fn min_diff(k: i32, sorted: &Vec<i32>) -> i32 {
    let mut low = 0;
    let mut high = sorted.len();
    while low < high {
        let mid = (low + high) / 2;
        if sorted[mid] == k {
            return 0;
        } else if sorted[mid] < k {
            low = mid + 1
        } else {
            high = mid;
        }
    }
    if low == sorted.len() {
        low -= 1;
    }
    let mut min = (k - sorted[low]).abs();
    if low > 0 {
        min = cmp::min(min, (k - sorted[low - 1]).abs());
    }
    if low < sorted.len() - 1 {
        min = cmp::min(min, (k - sorted[low + 1]).abs());
    }
    min
}

pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let diffs: Vec<i32> = (0..nums1.len())
        .map(|i| (nums1[i] - nums2[i]).abs())
        .collect();
    let mut sorted = nums1.clone();
    sorted.sort();
    let diffs_max: i32 = (0..nums1.len())
        .map(|i| diffs[i] - min_diff(nums2[i], &sorted))
        .max()
        .unwrap();
    let mut res = 0;
    res -= diffs_max;
    for i in 0..diffs.len() {
        res += diffs[i];
        res %= 1_000_000_007;
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
