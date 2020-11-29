/*
You are given an array nums of n positive integers.

You can perform two types of operations on any element of the array any number of times:

    If the element is even, divide it by 2.
        For example, if the array is [1,2,3,4], then you can do this operation on the last element, and the array will be [1,2,3,2].
    If the element is odd, multiply it by 2.
        For example, if the array is [1,2,3,4], then you can do this operation on the first element, and the array will be [2,2,3,4].

The deviation of the array is the maximum difference between any two elements in the array.

Return the minimum deviation the array can have after performing some number of operations.
*/

pub fn minimum_deviation(mut nums: Vec<i32>) -> i32 {
    loop {
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();
        if min % 2 == 0 && max % 2 == 1 {
            return max - min;
        }
        if let Some(max_odd) = nums.iter().filter(|&num| num % 2 == 1).max() {
            for i in 0..nums.len() {
                // while nums[i] % 2 == 0 && {
                //     while
                // }
            }
        };
        let min_even = nums.iter().filter(|&num| num % 2 == 0).min();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![1, 2, 3, 4], 1),
            (vec![4, 1, 5, 20, 3], 3),
            (vec![2, 10, 8], 3),
        ];
    }
}
