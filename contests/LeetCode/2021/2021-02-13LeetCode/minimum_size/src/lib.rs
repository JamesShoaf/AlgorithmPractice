/*
You are given an integer array nums where the ith bag contains nums[i] balls.
You are also given an integer maxOperations.

You can perform the following operation at most maxOperations times:

    Take any bag of balls and divide it into two new bags with a positive number of balls.
        For example, a bag of 5 balls can become two new bags of 1 and 4 balls, or two new
        bags of 2 and 3 balls.

Your penalty is the maximum number of balls in a bag. You want to minimize your penalty
after the operations.

Return the minimum possible penalty after performing the operations.
*/

/*
naive approach
for each potential penalty 1..nums.max
count the number of divisions it would take to divide a bag to no more than that sum
ceil(num / penalty) - 1
and sum the number of divisions. If the sum is less than maxOperations, return that penalty
O(w*n) time where w is the minimum bag maximum
*/

// pub fn minimum_size(mut nums: Vec<i32>, max_operations: i32) -> i32 {
//     nums.sort();
//     nums.reverse();
//     let nums: Vec<f64> = nums.into_iter().map(|num| num as f64).collect();
//     let mut penalty: f64 = 1.0;
//     let max_operations = max_operations as f64;
//     loop {
//         let mut sum = 0.0;
//         for i in 0..nums.len() {
//             sum += (nums[i] / penalty).ceil() - 1.0;
//             if sum > max_operations {
//                 break;
//             }
//         }
//         if sum <= max_operations {
//             return penalty as i32
//         }
//         penalty += 1.0;
//     }
// }

// O(mlog(n)) time where m is max_operations and n is nums.len()
// use std::collections::BinaryHeap;

// pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
//     if nums.is_empty() {
//         return 0;
//     }
//     // (max_bag, bag_count, original_val)
//     let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
//     for num in nums {
//         heap.push((num, 1, num));
//     }
//     for _ in 0..max_operations {
//         let (max_bag, bag_count, original_val) = heap.pop().unwrap();
//         if max_bag == 1 {
//             return 1;
//         }
//         let new_count = bag_count + 1;
//         let new_max = original_val / new_count + original_val % new_count;
//         heap.push((new_max, new_count, original_val));
//     }
//     heap.pop().unwrap().0
// }

// binary search along possible minimum bag maximums between 1 and nums.max()
// O(nlog(nums.max()))

pub fn minimum_size(mut nums: Vec<i32>, max_operations: i32) -> i32 {
    nums.sort();
    nums.reverse();
    let nums: Vec<f64> = nums.into_iter().map(|num| num as f64).collect();
    let mut low: f64 = 1.0;
    let mut high: f64 = nums[0];
    let max_operations = max_operations as f64;
    while low < high {
        let mid = ((low + high) / 2.0).floor();
        let mut sum = 0.0;
        for i in 0..nums.len() {
            sum += (nums[i] / mid).ceil() - 1.0;
            if sum > max_operations {
                break;
            }
        }
        if sum > max_operations {
            low = mid + 1.0;
        } else {
            high = mid;
        }
    }
    return low as i32;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
