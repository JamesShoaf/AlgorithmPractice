/* 
Given an integer array nums, find the contiguous subarray within an array (containing at least
one number) which has the largest product.
*/

fn max_product(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 { return 0; }
    use std::{ cmp, mem };
    let (mut max, mut min, mut best) = (1, 1, i32::MIN);
    for num in nums {
        if num < 0 {
            mem::swap(&mut max, &mut min);
        }
        max = cmp::max(max * num, num);
        min = cmp::min(min * num, num);
        best = cmp::max(best, max); 
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![2, 3, -2, 4], 6),
            (vec![-2, 0, -1], 0),
            (vec![-1], -1),
            (Vec::new(), 0),
            (vec![0], 0),
            (vec![1], 1),
            (vec![3, -1, 4], 4),
            (vec![3, -1, 4, 2], 8),
            (vec![-2, 0, -3, 0, -1, 0], 0),
            (vec![2, 0, 3, 0, 1, 0], 3),
            (vec![-1, 1, 1, 1, 1], 1),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(max_product(nums), expected);
        }
    }
}
