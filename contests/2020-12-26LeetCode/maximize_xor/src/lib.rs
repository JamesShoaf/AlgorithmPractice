/*
You are given an array nums consisting of non-negative integers. You are also given a queries array, where queries[i] = [xi, mi].

The answer to the ith query is the maximum bitwise XOR value of xi and any element of nums that does not exceed mi. In other words, the answer is max(nums[j] XOR xi) for all j such that nums[j] <= mi. If all elements in nums are larger than mi, then the answer is -1.

Return an integer array answer where answer.length == queries.length and answer[i] is the answer to the ith query.
*/

// Optimized solution - Binary Trie - O(nlog(n)) to assemble trie, O(mlog(m)) to traverse

// Naive O(m*n) time approach - iterate over each number
use std::cmp;

pub fn maximize_xor(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    nums.sort_unstable();
    nums.dedup();
    queries
        .into_iter()
        .map(|vec| {
            let mut xor = -1;
            let (x, m) = (vec[0], vec[1]);
            for i in 0..nums.len() {
                if nums[i] > m {
                    break;
                }
                xor = cmp::max(xor, x ^ nums[i]);
            }
            xor
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
