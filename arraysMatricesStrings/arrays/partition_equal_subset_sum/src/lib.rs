/*
Given a non-empty array nums containing only positive integers, find if the array can be
partitioned into two subsets such that the sum of elements in both subsets is equal.
*/

// Optimized DP solution (O(n*w) time, O(w) space) (w = Sum/2))
pub fn can_partition(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return true;
    }
    let sum: i32 = nums.iter().sum();
    if sum % 2 == 1 {
        return false;
    }
    let sum = sum as usize / 2;
    // dp[i] = whether a set of sum i is possible
    let mut dp = vec![false; sum + 1];
    // it's always possible to make a set of total sum 0
    dp[0] = true;
    for num in nums {
        let num = num as usize;
        for j in (num..=sum).rev() {
            dp[j] |= dp[j - num];
        }
        if dp[sum] {
            return true;
        }
    }
    false
}

// DP solution (O(n*w) time, O(n*w) space (w = Sum / 2))
// pub fn can_partition(nums: Vec<i32>) -> bool {
//     let sum: i32 = nums.iter().sum();
//     if sum % 2 == 1 {
//         return false;
//     }
//     let sum = sum as usize / 2;
//     let mut memo = vec![vec![false; sum + 1]; nums.len() + 1];
//     // it is possible to make a set with sum 0 using the first 0 elements.
//     memo[0][0] = true;
//     for element_count in 1..=nums.len() {
//         for target in 0..=sum {
//             let possible_wo_current_element = memo[element_count - 1][target];
//             let possible_w_current_element = if let Some(target_less_current) =
//                 target.checked_sub(nums[element_count - 1] as usize)
//             {
//                 memo[element_count - 1][target_less_current]
//             } else {
//                 false
//             };
//             memo[element_count][target] = possible_wo_current_element || possible_w_current_element;
//         }
//     }
//     memo[nums.len()][sum]
// }

// recursive solution (O(2^n) time, O(n) space)
// fn helper(current: i32, target: i32, index: usize, nums: &Vec<i32>) -> bool {
//     if current == target {
//         return true;
//     }
//     if current > target || index == nums.len() {
//         return false;
//     }
//     helper(current + nums[index], target, index + 1, nums)
//         || helper(current, target, index + 1, nums)
// }

// pub fn can_partition(nums: Vec<i32>) -> bool {
//     let sum: i32 = nums.iter().sum();
//     if sum % 2 == 1 {
//         return false;
//     }
//     helper(0, sum / 2, 0, &nums)
// }

// brute force (O(n2^n) time, O(2^n) space)
// use std::collections::HashSet;

// pub fn can_partition(nums: Vec<i32>) -> bool {
//     // optimization: if the sum of the set is odd, it cannot be divided evenly
//     if nums.iter().sum::<i32>() % 2 == 1 {
//         return false;
//     }
//     let mut set: HashSet<i32> = HashSet::new();
//     set.insert(0);
//     for i in 0..nums.len() {
//         let mut next_set: HashSet<i32> = HashSet::new();
//         for n in set.into_iter() {
//             next_set.insert(n + nums[i]);
//             next_set.insert(n - nums[i]);
//         }
//         set = next_set;
//         // println!("{:?}", set);
//     }
//     set.contains(&0)
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), true),
            (vec![0], true),
            (vec![5], false),
            (vec![5, 5], true),
            (vec![1, 5, 11, 5], true),
            (vec![1, 2, 3, 5], false),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(can_partition(nums), expected);
        }
    }
}
