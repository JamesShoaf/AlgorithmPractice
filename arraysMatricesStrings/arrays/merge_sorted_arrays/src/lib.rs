/*
Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.

The number of elements initialized in nums1 and nums2 are m and n respectively. You may assume that
nums1 has enough space (size that is equal to m + n) to hold additional elements from nums2.
*/

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut l = m - 1;
    let mut r = n - 1;
    for i in (0..(m + n) as usize).rev() {
        if r < 0 || l >= 0 && nums1[l as usize] > nums2[r as usize] {
            nums1[i] = nums1[l as usize];
            l -= 1;
        } else {
            nums1[i] = nums2[r as usize];
            r -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                vec![1, 2, 3, 0, 0, 0],
                3,
                vec![2, 5, 6],
                3,
                vec![1, 2, 2, 3, 5, 6],
            ),
            (vec![1], 1, Vec::new(), 0, vec![1]),
            (
                vec![4, 5, 6, 0, 0, 0],
                3,
                vec![1, 2, 3],
                3,
                vec![1, 2, 3, 4, 5, 6],
            ),
        ];
        for (mut nums1, m, mut nums2, n, expected) in test_tuples {
            merge(&mut nums1, m, &mut nums2, n);
            assert_eq!(nums1, expected);
        }
    }
}
