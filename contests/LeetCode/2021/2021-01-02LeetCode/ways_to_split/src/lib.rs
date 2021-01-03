/*
A split of an integer array is good if:

    The array is split into three non-empty contiguous subarrays - named left, mid, right
    respectively from left to right.
    The sum of the elements in left is less than or equal to the sum of the elements in mid, and
    the sum of the elements in mid is less than or equal to the sum of the elements in right.

Given nums, an array of non-negative integers, return the number of good ways to split nums. As the
number may be too large, return it modulo 109 + 7.
*/

// linear implementation (from quantuminfo) - Prefix sum + sliding window
pub fn ways_to_split(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0;
    }
    let mut prefix = vec![0];
    for i in 0..nums.len() {
        prefix.push(prefix[i] + nums[i]);
    }
    let mut res: u64 = 0;
    // i1 is the last invalid index for the middle subarray to start on
    let mut i1 = nums.len() - 2;
    // i2 is the last valid index for the middle subarray to start on
    let mut i2 = i1;
    let sum = *prefix.last().unwrap();
    // j is the starting index for the right subarray
    for j in (2..nums.len()).rev() {
        // sum of subarray, nums[j..]
        let right = sum - prefix[j];
        // move i1 left until nums[i1..j] > nums[j..]
        while i1 >= j || i1 > 0 && prefix[j] - prefix[i1] <= right {
            i1 -= 1;
        }
        // move i2 left until nums[..i2] < nums[i2..j]
        while i2 >= j || i2 > 0 && prefix[i2] > prefix[j] - prefix[i2] {
            i2 -= 1;
        }
        // count valid mid/left splits for given right split
        if i2 > i1 {
            res += (i2 - i1) as u64;
        }
    }
    (res % 1_000_000_0007) as i32
}

// initial implementation - Prefix sum + 2 binary searches for first and last possible mid starts
// const MODULO: usize = 1_000_000_007;

// pub fn ways_to_split(nums: Vec<i32>) -> i32 {
//     if nums.len() < 3 {
//         return 0;
//     }
//     let mut res = 0;
//     let left_sum: Vec<i32> = nums
//         .iter()
//         .scan(0, |state, &num| {
//             *state += num;
//             Some(*state)
//         })
//         .collect();
//     for i in 0..left_sum.len() {
//         if let Some(ways) = ways_to_split_mid_right(i, &left_sum) {
//             res += ways;
//             res %= MODULO;
//         }
//     }
//     res as i32
// }

fn ways_to_split_mid_right(i: usize, left_sum: &Vec<i32>) -> Option<usize> {
    let low_j = find_low_j(i + 1, left_sum[i] * 2, left_sum)?;
    let mid_max = (*left_sum.last().unwrap() - left_sum[i]) / 2 + left_sum[i];
    let high_j = find_high_j(low_j, mid_max, left_sum)?;
    Some(high_j - low_j + 1)
}

// find first index j such that nums[j] >= target
fn find_low_j(low_bound: usize, target: i32, nums: &Vec<i32>) -> Option<usize> {
    // println!("low_bound: {}", low_bound);
    let mut low = low_bound;
    let mut high = nums.len();
    while low < high {
        // print!("low: {}, high: {} \n", low, high);
        let mid = (low + high) / 2;
        if nums[mid] >= target {
            high = mid;
        }
        if nums[mid] < target {
            low = mid + 1;
        }
    }
    Some(low).filter(|_| low < nums.len() && nums[low] >= target)
}

// find last index j such that nums[j] <= target and j <= nums.len() - 2
use std::cmp;

fn find_high_j(low_bound: usize, target: i32, nums: &Vec<i32>) -> Option<usize> {
    let mut low = low_bound;
    let mut high = nums.len() - 1;
    while low < high {
        let mid = (low + high + 1) / 2;
        if nums[mid] > target {
            high = mid - 1;
        }
        if nums[mid] <= target {
            low = mid;
        }
    }
    Some(cmp::min(nums.len() - 2, high)).filter(|_| high >= low_bound && nums[high] <= target)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ways_to_split() {
        let test_tuples = vec![
            (vec![1, 1, 1], 1),
            (vec![1, 2, 2, 2, 5, 0], 3),
            (vec![3, 2, 1], 0),
            (vec![0, 0, 0, 0, 0, 0, 0], 15),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(super::ways_to_split(nums), expected);
        }
    }

    #[test]
    fn test_ways_to_split_mid_right() {
        let test_tuples = vec![
            (vec![1, 2, 3], vec![1, 0, 0]),
            (vec![1, 3, 5, 7, 12, 12], vec![2, 1, 0, 0, 0, 0]),
            (vec![3, 2, 1], vec![0, 0, 0]),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(
                (0..nums.len())
                    .map(|i| super::ways_to_split_mid_right(i, &nums).unwrap_or(0))
                    .collect::<Vec<usize>>(),
                expected
            );
        }
    }

    #[test]
    fn test_find_low_j() {
        let test_tuples = vec![(
            vec![1, 2, 3],
            vec![
                (0, 1, Some(0)),
                (1, 1, Some(1)),
                (2, 1, Some(2)),
                (3, 1, None),
                (0, 2, Some(1)),
                (0, 3, Some(2)),
                (0, 4, None),
            ],
        )];
        for (nums, test_inputs) in test_tuples {
            for (low_bound, target, expected) in test_inputs {
                assert_eq!(super::find_low_j(low_bound, target, &nums), expected);
            }
        }
    }

    #[test]
    fn test_find_high_j() {
        let test_tuples = vec![
            (vec![1, 2, 3], vec![(1, 2, Some(1)), (2, 2, None)]),
            (
                vec![1, 3, 5, 7, 12, 12],
                vec![
                    (1, 1, None),
                    (1, 3, Some(1)),
                    (1, 4, Some(1)),
                    (1, 5, Some(2)),
                    (1, 6, Some(2)),
                    (1, 7, Some(3)),
                    (1, 11, Some(3)),
                    (1, 12, Some(4)),
                    (1, 13, Some(4)),
                    (2, 1, None),
                    (2, 3, None),
                    (2, 4, None),
                    (2, 5, Some(2)),
                    (2, 6, Some(2)),
                    (2, 7, Some(3)),
                    (2, 11, Some(3)),
                    (2, 12, Some(4)),
                    (2, 13, Some(4)),
                    (3, 1, None),
                    (3, 3, None),
                    (3, 4, None),
                    (3, 5, None),
                    (3, 6, None),
                    (3, 7, Some(3)),
                    (3, 11, Some(3)),
                    (3, 12, Some(4)),
                    (3, 13, Some(4)),
                    (4, 1, None),
                    (4, 3, None),
                    (4, 4, None),
                    (4, 5, None),
                    (4, 6, None),
                    (4, 7, None),
                    (4, 11, None),
                    (4, 12, Some(4)),
                    (4, 13, Some(4)),
                    (4, 1, None),
                    (5, 3, None),
                    (5, 4, None),
                    (5, 5, None),
                    (5, 6, None),
                    (5, 7, None),
                    (5, 11, None),
                    (5, 12, Some(4)),
                    (5, 13, Some(4)),
                ],
            ),
        ];
        for (nums, test_inputs) in test_tuples {
            for (low_bound, target, expected) in test_inputs {
                assert_eq!(
                    super::find_high_j(low_bound, target, &nums),
                    expected,
                    "high_j inputs: low_j: {}, target: {}, nums: {:?}",
                    low_bound,
                    target,
                    nums
                );
            }
        }
    }
}
