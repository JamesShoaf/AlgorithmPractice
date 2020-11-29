/*
You are given an integer array nums of even length n and an integer limit. In one move, you can
replace any integer from nums with another integer between 1 and limit, inclusive.

The array nums is complementary if for all indices i (0-indexed), nums[i] + nums[n - 1 - i] equals
the same number. For example, the array [1,2,3,4] is complementary because for all indices i,
nums[i] + nums[n - 1 - i] = 5.

Return the minimum number of moves required to make nums complementary.
*/

pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![1, 2, 4, 3], 4, 1),
            (vec![1, 2, 2, 1], 3, 2),
            (vec![1, 2, 1, 2], 2, 0),
            (vec![1, 2, 3, 10, 10, 12], 3, 1),
            (vec![9, 8, 7, 10, 11, 12], 3, 3),
            (vec![9, 20, 4, 10, 88, 15], 3, 6),
        ];
        for (nums, limit, expected) in test_tuples {
            assert_eq!(min_moves(nums, limit), expected);
        }
    }
}
