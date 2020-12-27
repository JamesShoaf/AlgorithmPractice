/*
You are given an integer array, nums, and an integer k. nums comprises of only 0's and 1's. In one
move, you can choose two adjacent indices and swap their values.

Return the minimum number of moves required so that nums has k consecutive 1's.
*/

use std::cmp;
use std::collections::VecDeque;

fn min_moves_helper(mid: usize, queue: &VecDeque<usize>) -> usize {
    let mut res = 0;
    for i in 0..mid {
        res += queue[mid] + i - queue[i] - mid;
    }
    for i in mid + 1..queue.len() {
        res += queue[i] + mid - queue[mid] - i;
    }
    res
}

fn min_moves_for_subset(queue: &VecDeque<usize>) -> usize {
    if queue.len() < 2 {
        return 0;
    }
    let mid_right = queue.len() / 2;
    let mid_left = (queue.len() - 1) / 2;
    if mid_left != mid_right {
        cmp::min(
            min_moves_helper(mid_left, queue),
            min_moves_helper(mid_right, queue),
        )
    } else {
        min_moves_helper(mid_left, queue)
    }
}

pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut best = usize::MAX;
    for i in 0..nums.len() {
        if nums[i] == 1 {
            queue.push_back(i);
            if queue.len() > k {
                queue.pop_front();
            }
            if queue.len() == k {
                best = cmp::min(best, min_moves_for_subset(&queue))
            }
        }
    }
    best as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_moves() {
        let test_tuples = vec![
            (vec![1, 0, 0, 1, 0, 1], 2, 1),
            (vec![1, 0, 0, 0, 0, 0, 1, 1], 3, 5),
            (vec![1, 1, 0, 1], 2, 0),
            (vec![1, 1, 0, 0, 1, 1], 4, 4),
            (
                vec![
                    0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0,
                    1, 0,
                ],
                12,
                52,
            ),
        ];
        for (nums, k, expected) in test_tuples {
            assert_eq!(min_moves(nums, k), expected);
        }
    }

    #[test]
    fn test_min_moves_for_subset() {
        let test_tuples = vec![
            (vec![0, 3], 2),
            (vec![3, 5], 1),
            (vec![0, 6, 7], 5),
            (vec![0, 1, 4, 5], 4),
            (vec![1, 3, 4, 7, 12, 14, 15, 19, 22, 23, 24, 26], 52),
        ];
        for (indices, expected) in test_tuples {
            let queue = VecDeque::from(indices);
            assert_eq!(min_moves_for_subset(&queue), expected);
        }
    }

    #[test]
    fn test_min_moves_helper() {
        let test_tuples = vec![
            (vec![0, 3], 0, 2),
            (vec![0, 3], 1, 2),
            (vec![3, 5], 0, 1),
            (vec![3, 5], 1, 1),
            (vec![0, 6, 7], 1, 5),
            (vec![0, 1, 4, 5], 1, 4),
            (vec![0, 1, 4, 5], 2, 4),
            (vec![1, 3, 4, 7, 12, 14, 15, 19, 22, 23, 24, 26], 5, 52),
            (vec![1, 3, 4, 7, 12, 14, 15, 19, 22, 23, 24, 26], 6, 52),
        ];
        for (indices, mid, expected) in test_tuples {
            let queue = VecDeque::from(indices);
            assert_eq!(min_moves_helper(mid, &queue), expected);
        }
    }
}
