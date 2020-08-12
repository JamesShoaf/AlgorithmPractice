pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    use std::cmp::{min, max};
    if nums1.len() > nums2.len() { return find_median_sorted_arrays(nums2, nums1); }
    
    let mut low = 0;
    let mut high = nums1.len();
    while low <= high {
        let partition1 = (low + high) / 2;
        let partition2 = (nums1.len() + nums2.len() + 1) / 2 - partition1;

        let max_left_1 = if partition1 == 0 { i32::MIN } else { nums1[partition1 - 1] };
        let max_right_1 = if partition1 == nums1.len() { i32::MAX } else { nums1[partition1] };
        let max_left_2 = if partition2 == 0 { i32::MIN } else { nums2[partition2 - 1] };
        let max_right_2 = if partition2 == nums2.len() { i32::MAX } else { nums2[partition2] };

        if max_left_1 <= max_right_2 && max_left_2 <= max_right_1 {
            if (nums1.len() + nums2.len()) % 2 == 0 {
                return (max(max_left_1, max_left_2) as f64 + min(max_right_1, max_right_2) as f64) / 2.0;
            }
            return max(max_left_1, max_left_2) as f64;
        } else if max_left_1 > max_right_2 { high = partition1 - 1; }
        else { low = partition1 + 1; }
    }
    0.0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_single_sizes() {
        let test_tuples = vec![
            (
                vec![1],
                vec![2],
                1.5,
            ),    
            (
                vec![2],
                vec![1],
                1.5,
            ),    
            (
                vec![1, 3],
                vec![2],
                2.0,
            ),    
            (
                vec![0, 1],
                vec![2],
                1.0,
            ),    
            (
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                vec![2],
                3.0,
            ),    
            (
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
                vec![2],
                3.5,
            ),    
        ];
        for (nums1, nums2, expected) in test_tuples { 
            assert_eq!(super::find_median_sorted_arrays(nums1, nums2), expected);
        }
    }
    #[test]
    fn test_matching_sizes() {
        let test_tuples = vec![
            (
                vec![1, 3, 5],
                vec![1, 3, 5],
                3.0,
            ),
            (
                vec![0, 0, 1],
                vec![1, 3, 5],
                1.0,
            ),
            (
                vec![2, 3, 5],
                vec![0, 0, 1],
                1.5,
            ),
            (
                vec![1, 2],
                vec![3, 4],
                2.5,
            ),
        ];
        for (nums1, nums2, expected) in test_tuples { 
            assert_eq!(super::find_median_sorted_arrays(nums1, nums2), expected);
        }
    }
}
