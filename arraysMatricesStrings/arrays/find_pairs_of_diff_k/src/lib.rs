/* 
Given an array of integers nums and an integer k, return the number of unique k-diff pairs in the array.

A k-diff pair is an integer pair (nums[i], nums[j]), where the following are true:

    0 <= i, j < nums.length
    i != j
    a <= b
    b - a == k

*/

// move the pointer to the next value
fn progress_pointer(nums: &Vec<i32>, pointer: &mut usize) {
    let init = nums[*pointer];
    while *pointer < nums.len() && nums[*pointer] == init {
        *pointer += 1;
    }
}
// sort the array, then progress two pointers through nums to find pairs
pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    let mut pair_count = 0;
    let mut low = 0;
    let mut high = 1;
    while high < nums.len() && nums[high] - nums[low] < k {
        progress_pointer(&nums, &mut high);
    }
    // because high always >= low, we only need to check if high is in bounds
    while high < nums.len() {
        if low == high {
            high += 1;
        } else {
            if nums[high] - nums[low] == k {
                pair_count += 1;
                progress_pointer(&nums, &mut low);
                progress_pointer(&nums, &mut high);
            } else if nums[high] - nums[low] < k {
                progress_pointer(&nums, &mut high);
            } else {
                progress_pointer(&nums, &mut low);
            }
        }
    }
    pair_count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![3, 1, 4, 1, 5], 2, 2),
            (vec![1, 2, 3, 4, 5], 1, 4),
            (vec![1, 3, 1, 4, 5], 0, 1),
            (vec![1, 2, 4, 4, 3, 3, 0, 9, 2, 3], 3, 2),
            (vec![-1, -2, -3], 1, 2),
            (Vec::new(), 1, 0),
            (vec![1, 2], 1, 1),
            (vec![1, 2], 0, 0),
        ];
        for (nums, k, expected) in test_tuples {
            assert_eq!(find_pairs(nums, k), expected);
        }
    }
}
