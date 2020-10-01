/* 
Your are given an array of positive integers nums.

Count and print the number of (contiguous) subarrays where the product of all the elements in the
subarray is less than k.
*/

pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    if k <= 1 {
        return 0;
    }
    nums.iter()
    .enumerate()
    .scan((0, 1), |(i, running_total), (j, num)| {
        *running_total *= num;
        while *running_total >= k {
            *running_total /= nums[*i];
            *i += 1;
        }
        Some(if *i <= j { 1 + j - *i } else { 0 })
    }).fold(0, |acc, val| acc + val) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![10, 5, 2, 6], 100, 8),
            (vec![1, 1, 1, 1], 1, 0),
            (vec![10, 11, 12, 13], 8, 0),
            (Vec::new(), 8, 0),
            (vec![2, 2, 2, 2], 8, 7),
            (vec![2, 2, 2, 2], 9, 9),
            (vec![2, 2, 2, 2, 9], 9, 9),
            (vec![2, 2, 5, 2, 2], 5, 6),
        ];
        for (nums, k, expected) in test_tuples {
            assert_eq!(num_subarray_product_less_than_k(nums, k), expected);
        }
    }
}
