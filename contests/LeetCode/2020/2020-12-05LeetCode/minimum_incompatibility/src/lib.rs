/*
You are given an integer array nums​​​ and an integer k. You are asked to distribute this array
into k subsets of equal size such that there are no two equal elements in the same subset.

A subset's incompatibility is the difference between the maximum and minimum elements in that array.

Return the minimum possible sum of incompatibilities of the k subsets after distributing the array
optimally, or return -1 if it is not possible.

A subset is a group integers that appear in the array with no particular order.
*/

// initial backtracking solution
use std::cmp;

fn backtrack(
    i: usize,
    group_size: usize,
    nums: &Vec<i32>,
    memo: &mut Vec<Vec<i32>>,
) -> Option<i32> {
    // if all numbers have been successfully inserted, return the minimum_incompatibility
    if i == nums.len() {
        let mut sum = 0;
        for j in 0..memo.len() {
            sum += *memo[j].iter().max().unwrap() - *memo[j].iter().min().unwrap()
        }
        return Some(sum);
    }
    let mut res: Option<i32> = None;
    // sort nums[i] into each of the available subsets it fits in
    for j in 0..memo.len() {
        if memo[j].len() < group_size && !memo[j].contains(&nums[i]) {
            memo[j].push(nums[i]);
            if let Some(min) = backtrack(i + 1, group_size, nums, memo) {
                if let Some(prev) = res.as_mut() {
                    *prev = cmp::min(*prev, min);
                } else {
                    res = Some(min);
                }
            }
            memo[j].pop();
        }
        // stop after the first empty vector
        if memo[j].len() == 0 {
            break;
        }
    }
    res
}

pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut memo: Vec<Vec<i32>> = vec![Vec::new(); k];
    let group_size = nums.len() / k;
    if let Some(min) = backtrack(0, group_size, &nums, &mut memo) {
        min
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
