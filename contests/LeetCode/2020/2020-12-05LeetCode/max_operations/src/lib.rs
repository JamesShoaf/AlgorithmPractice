/*
You are given an integer array nums and an integer k.

In one operation, you can pick two numbers from the array whose sum equals k and remove them from
the array.

Return the maximum number of operations you can perform on the array.
*/

use std::cmp;
use std::collections::HashMap;

pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        *counter.entry(num).or_insert(0) += 1;
    }
    if k % 2 == 0 {
        if let Some(half_count) = counter.remove(&(k / 2)) {
            res += half_count / 2;
        }
    }
    for n in counter.keys().map(|&k| k).collect::<Vec<i32>>() {
        let count = counter.remove(&n).unwrap_or(0);
        if let Some(complement_count) = counter.remove(&(k - n)) {
            res += cmp::min(count, complement_count);
        }
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
