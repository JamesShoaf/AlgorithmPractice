/*
You are given two arrays of integers nums1 and nums2, possibly of different lengths. The values
in the arrays are between 1 and 6, inclusive.

In one operation, you can change any integer's value in any of the arrays to any value between
1 and 6, inclusive.

Return the minimum number of operations required to make the sum of values in nums1 equal to
the sum of values in nums2. Return -1​​​​​ if it is not possible to make the sum of the two arrays
equal.
*/

use std::cmp;

fn helper(mut sum1: i32, mut counts1: [i32; 6], mut sum2: i32, mut counts2: [i32; 6]) -> i32 {
    if sum2 > sum1 {
        return helper(sum2, counts2, sum1, counts1);
    }
    let mut operations = 0;
    let mut i = 5;
    let mut j = 0;
    let mut change = 5;
    while sum1 > sum2 {
        let diff = (sum1 - sum2) / change;
        let subs = cmp::min(counts1[i], diff);
        sum1 -= subs * change;
        counts1[i] -= subs;
        operations += subs;
        let diff = (sum1 - sum2) / change;
        let adds = cmp::min(counts2[j], diff);
        sum2 += adds * change;
        counts2[j] -= adds;
        operations += adds;
        if sum1 != sum2 && sum1 - sum2 < change && (counts1[i] > 0 || counts2[j] > 0) {
            return operations + 1;
        }
        i -= 1;
        j += 1;
        change -= 1;
    }
    operations
}

pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    if cmp::max(nums1.len(), nums2.len()) - cmp::min(nums1.len(), nums2.len())
        > 5 * cmp::min(nums1.len(), nums2.len())
    {
        return -1;
    }
    let sum1: i32 = nums1.iter().sum();
    let sum2: i32 = nums2.iter().sum();
    let mut counts1 = [0; 6];
    for num in nums1 {
        counts1[num as usize - 1] += 1;
    }
    let mut counts2 = [0; 6];
    for num in nums2 {
        counts2[num as usize - 1] += 1;
    }
    helper(sum1, counts1, sum2, counts2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
