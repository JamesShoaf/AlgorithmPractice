fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    let mut first_odd = 0;
    for i in 0..nums.len() {
        if nums[i] % 2 == 0 {
            nums.swap(i, first_odd);
            first_odd += 1;
        }
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), Vec::new()),
            (vec![1, 2, 3, 4], vec![2, 4, 3, 1]),
            (vec![1, 3, 5, 7, 2, 4, 6], vec![2, 4, 6, 7, 1, 3, 5]),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(sort_array_by_parity(nums), expected);
        }
    }
}
