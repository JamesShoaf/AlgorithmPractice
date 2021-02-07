/*
Given an array nums, return true if the array was originally sorted in non-decreasing order,
then rotated some number of positions (including zero).Otherwise, return false.

There may be duplicates in the original array.

Note: An array A rotated by x positions results in an array B of the same length such that
A[i] == B[(i+x) % A.length], where % is the modulo operation.
*/

pub fn check(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return false;
    }
    let mut inversion_count = 0;
    for i in 1..nums.len() {
        if nums[i - 1] > nums[i] {
            inversion_count += 1;
        }
    }
    if *nums.last().unwrap() > *nums.first().unwrap() {
        inversion_count += 1;
    }
    inversion_count < 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
