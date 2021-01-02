/*
You are given an array nums of n positive integers.

You can perform two types of operations on any element of the array any number of times:

    If the element is even, divide it by 2.
        For example, if the array is [1,2,3,4], then you can do this operation on the last element, and the array will be [1,2,3,2].
    If the element is odd, multiply it by 2.
        For example, if the array is [1,2,3,4], then you can do this operation on the first element, and the array will be [2,2,3,4].

The deviation of the array is the maximum difference between any two elements in the array.

Return the minimum deviation the array can have after performing some number of operations.
*/

/*
if 2x > y && y > x then (with some rearrangement) 2x - y > y - x.
if 2x > y && x > y then 2x - y > x - y.
Therefore, halving the maximum will always reduce its distance from the minimum

Assuming x >= 1:
Consider an array [2x, x + 1, 2x - 1]:
x + 1 is the minimum and the maximum difference is x - 1.

Halving the greatest value, 2x, results in [x, x + 1, 2x - 1]
x is now the minimum, but the maximum difference is still 2x - 1.
*/

use std::cmp;
use std::collections::BinaryHeap;

// Bump all odd numbers up to their even complement
// Half the largest element of the array repeatedly until the largest element is odd
// Compare this number (the effective maximum) with the minimum (after all operations)
pub fn minimum_deviation(mut nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return 0;
    }
    for num in nums.iter_mut().filter(|num| **num % 2 == 1) {
        *num *= 2;
    }
    let mut min = *nums.iter().min().unwrap();
    let mut max_heap: BinaryHeap<i32> = BinaryHeap::from(nums);
    let mut res = i32::MAX;
    while let Some(mut max) = max_heap.pop() {
        res = cmp::min(res, max - min);
        if max % 2 == 1 {
            return res;
        }
        max /= 2;
        min = cmp::min(min, max);
        max_heap.push(max);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), 0),
            (vec![1024], 0),
            (vec![1023], 0),
            (vec![1, 2, 3, 4], 1),
            (vec![4, 1, 5, 20, 3], 3),
            (vec![2, 10, 8], 3),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(minimum_deviation(nums), expected);
        }
    }
}
