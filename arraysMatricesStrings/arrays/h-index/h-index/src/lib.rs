fn h_index(nums: Vec<i32>) -> i32 {
    let mut counts: Vec<i32> = vec![0;nums.len() + 1];
    let len = nums.len();
    for num in nums {
        counts[std::cmp::min(num as usize, len)] += 1;
    }
    let mut sum = 0;
    for i in (0..counts.len()).rev() {
        sum += counts[i];
        if sum >= i as i32 { return i as i32; }
    }
    0
}

// O(1) space, O(nlog(n)) time
fn h_index_sort(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let n = nums.len();
    if n == 0 { return 0; }
    let mut low = 0;
    let mut high = n - 1;
    if nums[high] == 0 { return 0; }
    loop {
        let mid = (low + high) / 2;
        let mid_val = nums[mid];
        let mid_or_greater = (n - mid) as i32;
        if mid_val == mid_or_greater { return mid_or_greater; }
        else if mid_val < mid_or_greater {
            if mid == low { return (n - high) as i32; }
            low = mid;
        } else if mid_val > mid_or_greater {
            if mid == high { return (n - low) as i32; }
            high = mid;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let test_tuples = vec![
            (vec![3, 0, 6, 1, 5], 3),
            (Vec::new(), 0),
            (vec![3], 1),
            (vec![3, 3], 2),
            (vec![3, 3, 3], 3),
            (vec![i32::MAX; 5], 5),
        ];
        for (nums, expected) in test_tuples.clone() { assert_eq!(super::h_index(nums), expected); }
        for (nums, expected) in test_tuples { assert_eq!(super::h_index_sort(nums), expected); }
    }
}
