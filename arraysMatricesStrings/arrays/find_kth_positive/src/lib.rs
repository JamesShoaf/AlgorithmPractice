pub fn find_kth_positive(nums: Vec<i32>, k: i32) -> i32 {
    let mut current = 1;
    let mut missing_count = 0;
    for num in nums {
        while current < num {
            missing_count += 1;
            if missing_count == k {
                return current;
            }
            current += 1;
        }
        current += 1;
    }
    current + k - missing_count - 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![2, 3, 4, 7, 11], 1, 1),
            (vec![2, 3, 4, 7, 11], 2, 5),
            (vec![2, 3, 4, 7, 11], 3, 6),
            (vec![2, 3, 4, 7, 11], 4, 8),
            (vec![2, 3, 4, 7, 11], 5, 9),
            (vec![1, 2, 3, 4], 2, 6),
            (vec![1, 2, 3, 4], 10, 14),
            (vec![1, 4], 10, 12),
        ];
        for (nums, k, expected) in test_tuples {
            assert_eq!(super::find_kth_positive(nums, k), expected);
        }
    }
}
