use std::cmp;

struct Solution {}

// given a sorted array that has been rotated at an unknown pivot, but is otherwise non-decreasing,
// return the minimum element.
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 { panic!("Invalid input: Received vec with length 0"); }
        let mut low: usize = 0;
        let mut high: usize = nums.len() - 1;
        loop {
            // low will only be the lower than high if it is the lowest value
            if nums[low] < nums[high] { return nums[low]; }
            // if it is not, and there are only two elements left, return the other value
            if high - low <= 1 { return nums[high];}
            // pick a midpoint
            let mid: usize = (((low + high) / 2) as f32).floor() as usize;
            // if low is lower than the midpoint, the pivot lies after the midpoint
            if nums[low] < nums[mid] { low = mid + 1; }
            // if low and the midpoint are the same, either:
            // 1. The array contains identical values throughout
            // 2. The array contains identical values between low and mid
            // 3. The pivot is between low and mid
            else if nums[low] == nums[mid] {
                low = loop {
                    // in case 1 and 2, return to the binary search
                    if low == mid { break low + 1 }
                    low += 1;
                    // in case 3, low will reach a different value than mid
                    // if mid is lower than low, mid and low were both already the lowest value
                    // if the lowest value lies between them, low is now the lowest value
                    if nums[low] != nums[mid] { return cmp::min(nums[low], nums[mid]); }
                }
            }
            // if low is higher than the midpoint, the pivot lies between low and mid
            else { high = mid; }
        }
    }
}

fn main() {
    let nums: Vec<i32> = vec![2, 2, 2, 3, 3, 1, 1, 1, 1, 2];
    println!("nums: {:#?}", nums);
    println!("min: {}", Solution::find_min(nums));
}
