/*
Given a sorted (in ascending order) integer array nums of n elements and a target value, write a
function to search target in nums. If target exists, then return its index, otherwise return -1.
*/

fn helper(nums: &Vec<i32>, target: i32) -> Option<usize> {
    if !nums.is_empty() {
        let mut low = 0;
        let mut high = nums.len();
        while low <= high {
            let mid = (high + low) / 2;
            if nums[mid] == target {
                return Some(mid);
            }
            if nums[mid] > target {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
            if nums[mid] < target {
                if mid == nums.len() - 1 {
                    break;
                }
                low = mid + 1;
            }
        }
    }
    None
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if let Some(num) = helper(&nums, target) {
        return num as i32;
    }
    -1
}

// idiomatic method
/*
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if let Ok(x) = nums.binary_search(&target) {
        return x as i32;
    } else {return -1;}
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), 0, -1),
            (vec![0], -1, -1),
            (vec![0], 0, 0),
            (vec![0], 1, -1),
            (vec![-1, 0, 3, 5, 9, 12], -1, 0),
            (vec![-1, 0, 3, 5, 9, 12], 0, 1),
            (vec![-1, 0, 3, 5, 9, 12], 3, 2),
            (vec![-1, 0, 3, 5, 9, 12], 5, 3),
            (vec![-1, 0, 3, 5, 9, 12], 9, 4),
            (vec![-1, 0, 3, 5, 9, 12], 12, 5),
            (vec![-1, 0, 3, 5, 9, 12], -2, -1),
            (vec![-1, 0, 3, 5, 9, 12], 4, -1),
            (vec![-1, 0, 3, 5, 9, 12], 13, -1),
            (vec![-1, 0, 3, 4, 5, 9, 12], -1, 0),
            (vec![-1, 0, 3, 4, 5, 9, 12], 0, 1),
            (vec![-1, 0, 3, 4, 5, 9, 12], 3, 2),
            (vec![-1, 0, 3, 4, 5, 9, 12], 4, 3),
            (vec![-1, 0, 3, 4, 5, 9, 12], 5, 4),
            (vec![-1, 0, 3, 4, 5, 9, 12], 9, 5),
            (vec![-1, 0, 3, 4, 5, 9, 12], 12, 6),
            (vec![-1, 0, 3, 4, 5, 9, 12], -2, -1),
            (vec![-1, 0, 3, 4, 5, 9, 12], 6, -1),
            (vec![-1, 0, 3, 4, 5, 9, 12], 13, -1),
        ];
        for (nums, target, expected) in test_tuples {
            assert_eq!(search(nums, target), expected);
        }
    }
}
