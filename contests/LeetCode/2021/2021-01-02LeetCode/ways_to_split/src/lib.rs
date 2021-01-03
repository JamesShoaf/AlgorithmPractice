/*
A split of an integer array is good if:

    The array is split into three non-empty contiguous subarrays - named left, mid, right
    respectively from left to right.
    The sum of the elements in left is less than or equal to the sum of the elements in mid, and
    the sum of the elements in mid is less than or equal to the sum of the elements in right.

Given nums, an array of non-negative integers, return the number of good ways to split nums. As the
number may be too large, return it modulo 109 + 7.
*/

pub fn ways_to_split(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0;
    }
    let modulo = (10_i32.pow(9) + 7) as usize;
    let mut res = 0;
    let left_sum: Vec<i32> = nums
        .iter()
        .scan(0, |state, &num| {
            *state += num;
            Some(*state)
        })
        .collect();
    let max = *left_sum.last().unwrap();
    for i in 1..left_sum.len() {
        // binary search for first and last j such that 0..=i < i + 1..=j < j+1..nums.len()
        // find lowest j first
        if let Some(low_j) = binary_search(left_sum[i] * 2, i + 1, &left_sum) {
            if let Some(high_j) = binary_search(max - left_sum[low_j], low_j, &nums) {
                res += high_j - low_j + 1;
                res %= modulo;
            }
        }
    }
    res as i32
}

fn binary_search(target: i32, low_bound: usize, nums: &Vec<i32>) -> Option<usize> {
    let mut low = low_bound + 1;
    let mut high = nums.len();
    while low < high {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
