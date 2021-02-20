/*
You are given a 2D integer array groups of length n. You are also given an integer array nums.

You are asked if you can choose n disjoint subarrays from the array nums such that the ith
subarray is equal to groups[i] (0-indexed), and if i > 0, the (i-1)th subarray appears before
the ith subarray in nums (i.e. the subarrays must be in the same order as groups).

Return true if you can do this task, and false otherwise.

Note that the subarrays are disjoint if and only if there is no index k such that nums[k]
belongs to more than one subarray. A subarray is a contiguous sequence of elements within
an array.
 */

fn recursive(
    group_index: usize,
    nums_index: usize,
    groups: &Vec<Vec<i32>>,
    nums: &Vec<i32>,
) -> bool {
    if group_index == groups.len() {
        return true;
    }
    let group_len = groups[group_index].len();
    for i in nums_index..nums.len() {
        if i + group_len <= nums.len()
            && (0..group_len).all(|j| nums[i + j] == groups[group_index][j])
        {
            return recursive(group_index + 1, i + group_len, groups, nums);
        }
    }
    false
}

pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
    recursive(0, 0, &groups, &nums)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
