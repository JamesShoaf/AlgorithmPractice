// Given an unsorted integer array, find the smallest missing positive integer.
// Your algorithm should run in O(n) time and uses constant extra space.

pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        if nums[i] < 1 {
            nums[i] = nums.len() as i32 + 1;
        }
    }
    for i in 0..nums.len() {
        if nums[i].abs() > 0 && nums[i].abs() <= nums.len() as i32 {
            let j = nums[i].abs() as usize - 1;
            if nums[j] > 0 {
                nums[j] *= -1;
            }
        }
    }
    let mut missing = 1;
    while missing <= nums.len() {
        if nums[missing - 1] > 0 {
            break;
        }
        missing += 1;
    }
    missing as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![1, 2, 0], 3),
            (vec![3, 4, -1, 1], 2),
            (vec![7, 8, 9, 11, 12], 1),
            (Vec::new(), 1),
            (vec![1], 2),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(first_missing_positive(nums), expected);
        }
    }
}
