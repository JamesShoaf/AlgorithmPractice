/*
You are given an integer array nums and an integer x. In one operation, you can either remove the
leftmost or the rightmost element from the array nums and subtract its value from x. Note that
this modifies the array for future operations.

Return the minimum number of operations to reduce x to exactly 0 if it's possible, otherwise,
return -1.
*/

use std::cmp;

fn min_pre_and_postfix_element(nums: &Vec<i32>, mut x: i32) -> Option<usize> {
    if x == 0 {
        return Some(0);
    }
    if x < 0 || nums.is_empty() {
        return None;
    }
    let mut res = None;
    let len = nums.len();
    let mut r = len;
    while r > 0 && x > 0 {
        r -= 1;
        x -= nums[r];
    }
    if x == 0 {
        res = Some(len - r);
    }
    let mut l = 0;
    while r < len {
        x += nums[r];
        r += 1;
        while l < len && x > 0 {
            x -= nums[l];
            l += 1;
        }
        if l < r && x == 0 {
            let size = l + len - r;
            res = Some(cmp::min(res.unwrap_or(size), size));
        }
    }
    res
}

pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    if let Some(res) = min_pre_and_postfix_element(&nums, x) {
        return res as i32;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![1, 1, 4, 2, 3], 5, 2),
            (vec![5, 6, 7, 8, 9], 4, -1),
            (vec![3, 2, 20, 1, 1, 3], 10, 5),
            (vec![500, 1, 4, 2, 3], 500, 1),
            (vec![1, 1], 3, -1),
        ];
        for (nums, x, expected) in test_tuples {
            assert_eq!(min_operations(nums, x), expected);
        }
    }
}
