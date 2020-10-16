// Iterate through two copies of the original vector at an offset
// O(n) time, O(n) space
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    if nums.len() < 2 { return; }
    let concat = [&nums[..],&nums[..]].concat();
    let offset = nums.len() - k as usize % nums.len();
    for i in 0..nums.len() {
        nums[i] = concat[i + offset]
    }
}

// reverses the order of values within the range of [start, end)
fn reverse_range(nums: &mut Vec<i32>, start: usize, end: usize) {
    if start >= nums.len() { panic!("starting index out of bounds"); }
    if end > nums.len() { panic!("ending index out of bounds"); }
    if start >= end { return; }
    for i in 0..(end - start) / 2 {
        nums.swap(start + i, end - i - 1);
    }
}

// reverse the entire vector, then reverse the section before the break, and after
pub fn rotate_ii(nums: &mut Vec<i32>, k: i32) {
    if nums.len() < 2 { return; }
    reverse_range(nums, 0, nums.len());
    reverse_range(nums, 0, k as usize % nums.len());
    reverse_range(nums, k as usize % nums.len(), nums.len());
}

// gcd will tell us how many parallel cycles of offset k exist
// Stein's Binary GCD algorithm
fn gcd(a: usize, b: usize) -> usize {
    if b > a { return gcd(b, a); }
    if a == b { return a; }
    if b == 0 { return a; }
    if a & 1 == 0 {
        if b & 1 == 0 { return gcd(a >> 1, b >> 1) << 1; }
        return gcd(a >> 1, b);
    }
    if a & 1 == 0 { return gcd(a, b >> 1); }
    gcd(a - b, b)
}

use std::mem;

pub fn rotate_iii(nums: &mut Vec<i32>, k: i32) {
    if nums.len() < 2 { return; }
    let k = k as usize % nums.len();
    let cycle_count = gcd(nums.len(), k);
    if cycle_count == 0 { return; }
    let cycle_len = nums.len() / cycle_count;
    for i in 0..cycle_count {
        let mut index = i;
        let mut temp = nums[index];
        for _ in 0..cycle_len {
            let next_index = (index + k) % nums.len();
            mem::swap(&mut nums[next_index], &mut temp);
            index = next_index;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn generate_test_rotations() -> Vec<(Vec<i32>, i32, Vec<i32>)> {
        vec![
            (Vec::new(), 3, Vec::new()),
            (vec![1], 3, vec![1]),
            (vec![1, 2], 0, vec![1, 2]),
            (vec![1, 2], 1, vec![2, 1]),
            (vec![1, 2], 2, vec![1, 2]),
            (vec![1, 2], 3, vec![2, 1]),
            (vec![1, 2, 3, 4], 0, vec![1, 2, 3, 4]),
            (vec![1, 2, 3, 4], 1, vec![4, 1, 2, 3]),
            (vec![1, 2, 3, 4], 2, vec![3, 4, 1, 2]),
            (vec![1, 2, 3, 4], 3, vec![2, 3, 4, 1]),
            (vec![1, 2, 3, 4, 5, 6], 0, vec![1, 2, 3, 4, 5, 6]),
            (vec![1, 2, 3, 4, 5, 6], 1, vec![6, 1, 2, 3, 4, 5]),
            (vec![1, 2, 3, 4, 5, 6], 2, vec![5, 6, 1, 2, 3, 4]),
            (vec![1, 2, 3, 4, 5, 6], 3, vec![4, 5, 6, 1, 2, 3]),
            (vec![1, 2, 3, 4, 5, 6], 4, vec![3, 4, 5, 6, 1, 2]),
            (vec![1, 2, 3, 4, 5, 6], 5, vec![2, 3, 4, 5, 6, 1]),
            (vec![1, 2, 3, 4, 5, 6, 7], 0, vec![1, 2, 3, 4, 5, 6, 7]),
            (vec![1, 2, 3, 4, 5, 6, 7], 1, vec![7, 1, 2, 3, 4, 5, 6]),
            (vec![1, 2, 3, 4, 5, 6, 7], 2, vec![6, 7, 1, 2, 3, 4, 5]),
            (vec![1, 2, 3, 4, 5, 6, 7], 3, vec![5, 6, 7, 1, 2, 3, 4]),
            (vec![1, 2, 3, 4, 5, 6, 7], 4, vec![4, 5, 6, 7, 1, 2, 3]),
            (vec![1, 2, 3, 4, 5, 6, 7], 5, vec![3, 4, 5, 6, 7, 1, 2]),
            (vec![1, 2, 3, 4, 5, 6, 7], 6, vec![2, 3, 4, 5, 6, 7, 1]),
        ]
    }

    #[test]
    fn test_rotate() {
        for (mut nums, k, expected) in generate_test_rotations() {
            rotate(&mut nums, k);
            assert_eq!(nums, expected);
        }
    }

    #[test]
    fn test_reverse() {
        let test_tuples = vec![
            (vec![1, 2, 3, 4, 5], 0, 5, vec![5, 4, 3, 2, 1]),
            (vec![1, 2, 3, 4, 5], 0, 3, vec![3, 2, 1, 4, 5]),
            (vec![1, 2, 3, 4, 5], 3, 5, vec![1, 2, 3, 5, 4]),
        ];
        for (mut nums, start, end, expected) in test_tuples {
            reverse_range(&mut nums, start, end);
            assert_eq!(nums, expected);
        }
    }

    #[test]
    fn test_rotate_ii() {
        for (mut nums, k, expected) in generate_test_rotations() {
            rotate_ii(&mut nums, k);
            assert_eq!(nums, expected);
        }
    }

    #[test]
    fn test_rotate_iii() {
        for (mut nums, k, expected) in generate_test_rotations() {
            rotate_iii(&mut nums, k);
            assert_eq!(nums, expected);
        }
    }
}
