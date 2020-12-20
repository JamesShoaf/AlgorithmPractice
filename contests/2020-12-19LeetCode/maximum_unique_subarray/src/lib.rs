/*
You are given an array of positive integers nums and want to erase a subarray containing unique
elements. The score you get by erasing the subarray is equal to the sum of its elements.

Return the maximum score you can get by erasing exactly one subarray.

An array b is called to be a subarray of a if it forms a contiguous subsequence of a, that is, if
it is equal to a[l],a[l+1],...,a[r] for some (l,r).
*/

use std::cmp;
use std::collections::HashSet;

pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut current = 0;
    let mut set: HashSet<i32> = HashSet::new();
    let mut left = 0;
    for right in 0..nums.len() {
        while set.contains(&nums[right]) {
            current -= nums[left];
            set.remove(&nums[left]);
            left += 1;
        }
        set.insert(nums[right]);
        current += nums[right];
        max = cmp::max(max, current);
    }
    max
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
