/* 
Given an array of integers nums, sort the array in increasing order based on the frequency of the
values. If multiple values have the same frequency, sort them in decreasing order.
*/

use std::collections::HashMap;
use std::cmp::Reverse;

pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }
    let mut counts: Vec<(i32, usize)> = counts.into_iter().collect();
    counts.sort_by_key(|&(val, count)| (count, Reverse(val)));
    counts.into_iter()
        .map(|(val, count)| vec![val; count])
        .flatten()
        .collect()
} 

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
