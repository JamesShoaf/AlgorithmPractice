/*
You are given an integer array nums. The absolute sum of a subarray
[numsl, numsl+1, ..., numsr-1, numsr] is abs(numsl + numsl+1 + ... + numsr-1 + numsr).

Return the maximum absolute sum of any (possibly empty) subarray of nums.

Note that abs(x) is defined as follows:

    If x is a negative integer, then abs(x) = -x.
    If x is a non-negative integer, then abs(x) = x.

*/

use std::cmp;

pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut best: i32 = 0;
    let mut current_best = 0;
    let mut worst: i32 = 0;
    let mut current_worst = 0;
    for num in nums {
        current_best = cmp::max(0, current_best + num);
        best = cmp::max(best, current_best);
        current_worst = cmp::min(0, current_worst + num);
        worst = cmp::min(worst, current_worst);
    }
    cmp::max(best, worst.abs())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
