// You are given two sorted arrays of distinct integers nums1 and nums2.

// A valid path is defined as follows:

//     Choose array nums1 or nums2 to traverse (from index-0).
//     Traverse the current array from left to right.
//     If you are reading any value that is present in nums1 and nums2 you are allowed to change your path to the other array. (Only one repeated value is considered in the valid path).

// Score is defined as the sum of uniques values in a valid path.

// Return the maximum score you can obtain of all possible valid paths.

// Since the answer may be too large, return it modulo 10^9 + 7.

struct Solution {}

impl Solution {
    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        use std::cmp::max;
        let mod_val = 10_i32.pow(9) + 7;
        let mut set1: HashSet<i32> = HashSet::new();
        for num in nums1.clone() { set1.insert(num); }
        let mut set2: HashSet<i32> = HashSet::new();
        for num in nums2.clone() { set2.insert(num); }
        
        let mut iter1 = nums1.iter();
        let mut path1 = Vec::new();
        let mut path1_score = 0;
        let mut path1_mod = 0;
        while let Some(val) = iter1.next() {
            path1_score += val;
            if path1_score >= mod_val {
                path1_score -= mod_val;
                path1_mod += 1
            }
            if set2.contains(&val) {
                path1.push((path1_mod, path1_score));
                path1_score = 0;
                path1_mod = 0;
            }
        }
        path1.push((path1_mod, path1_score));
        
        let mut iter2 = nums2.iter();
        let mut path2 = Vec::new();
        let mut path2_score = 0;
        let mut path2_mod = 0;
        while let Some(val) = iter2.next() {
            path2_score += val;
            if path2_score >= mod_val {
                path2_score -= mod_val;
                path2_mod += 2
            }
            if set1.contains(&val) {
                path2.push((path2_mod, path2_score));
                path2_score = 0;
                path2_mod = 0;
            }
        }
        path2.push((path2_mod, path2_score));

        let mut output: i32 = 0;
        for i in 0..path1.len() {
            let (p1m, p1) = path1[i];
            let (p2m, p2) = path2[i];
            if p1m > p2m { output += p1; }
            else if p2m > p1m { output += p2; }
            else { output += max(p1, p2); }
            output %= mod_val
        }

        output
    }
}

fn main() {
    let test_tuples = vec![
        (vec![2, 4, 5, 8, 10], vec![4, 6, 8, 9], 30),
        (vec![1, 3, 5, 7, 9], vec![3, 5, 100], 109),
        (vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], 40),
        (vec![1, 4, 5, 8, 9, 11, 19], vec![2, 3, 4, 11, 12], 61),
    ];
    for (nums1, nums2, expected) in test_tuples {
        assert_eq!(Solution::max_sum(nums1, nums2), expected);
    }
}
