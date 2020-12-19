/*
Given an integer array nums, return true if there exists a triple of indices (i, j, k) such that
i < j < k and nums[i] < nums[j] < nums[k]. If no such indices exists, return false.
*/

pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut low = i32::MAX;
    let mut mid = i32::MAX;
    for num in nums {
        if num < low {
            low = num;
        }
        if num != low && num < mid {
            mid = num;
        }
        if num > mid {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
