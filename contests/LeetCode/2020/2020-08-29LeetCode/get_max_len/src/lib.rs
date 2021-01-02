/* 
Given an array of integers nums, find the maximum length of a subarray where the product of all its elements is positive.

A subarray of an array is a consecutive sequence of zero or more values taken out of that array.

Return the maximum length of a subarray with positive product.
*/

fn get_max_len(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    let len = nums.len();
    // first index in current nonzero run of positive sign, and of negative sign
    // initialize to len to indicate no matching signs
    let mut indexes = (len, len);
    let mut max_len = 0;
    for i in 0..len {
        // if the current value is zero, reset both indexes (no positive products possible)
        if nums[i] == 0 {
            indexes = (len, len)
        }
        // otherwise, set the matching index to i if it had been reset
        else if indexes.0 == len {
            indexes = (i, indexes.1);
        };
        // if the current number is negative, swap the two values (the product of two negatives is positive)
        if nums[i] < 0 {
            indexes = (indexes.1, indexes.0);
        }
        // finally, update the max length
        if i >= indexes.0 {
            max_len = max(max_len, 1 + i - indexes.0);
        }
    }
    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), 0),
            (vec![1, -2, -3, 4], 4),
            (vec![0, 1, -2, -3, -4], 3),
            (vec![-1, -2, -3, 0, 1], 2),
            (vec![-1, 2], 1),
            (vec![1, 2, 3, 5, -6, 4, 0, 10], 4),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(get_max_len(nums), expected);
        }
    }
}
