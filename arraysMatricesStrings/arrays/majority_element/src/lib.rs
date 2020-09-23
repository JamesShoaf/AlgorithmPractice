/* Given an integer array of size n, find all elements that appear more than ⌊ n/3 ⌋ times.

Note: The algorithm should run in linear time and in O(1) space. */

// This is a modified version of the Boyer-Moore algorithm
// https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm
// It finds the first and second most frequent elements by treating them as a combined majority element
// then confirms that the two elements do meet the criteria of count > floor(n/3)

fn confirm_majority(candidate: i32, nums: &Vec<i32>, output: &mut Vec<i32>) {
    if nums.iter().fold(0, |count, &num| {
            if num == candidate { count + 1 }
            else { count }
        }) > nums.len() / 3 {
        output.push(candidate);
    }
}

pub fn majority_element_ii(nums: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    let mut candidate1 = 0;
    let mut count1 = 0;
    // the choice of initial values for the candidates is arbitrary, but they need to be distinct
    let mut candidate2 = 1;
    let mut count2 = 0;
    for &num in nums.iter() {
        if candidate1 == num {
            count1 += 1;
        } else if candidate2 == num  {
            count2 += 1;
        } else if count1 == 0 {
            candidate1 = num;
            count1 = 1;
        } else if count2 == 0 {
            candidate2 = num;
            count2 = 1;
        } else {
            count1 -= 1;
            count2 -=1;
        }
    }
    confirm_majority(candidate1, &nums, &mut output);
    confirm_majority(candidate2, &nums, &mut output);
    output.sort();
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_majority_element_ii() {
        let test_tuples = vec![
            (vec![1, 2, 3], Vec::new()),
            (vec![0, 0, 0], vec![0]),
            (vec![3, 2, 3], vec![3]),
            (vec![1, 2, 3, 1, 2, 3, 1, 2], vec![1, 2]),
            (vec![1, 1, 1, 3, 3, 2, 2, 2], vec![1, 2]),
            (vec![1, 1, 1, 2, 3, 4, 2, 2], vec![1, 2]),
            (vec![1, 3, 4, 3, 2, 1, 1, 1, 2, 2, 2], vec![1, 2]),
            (vec![1, 3, 2, 3, 4, 1, 1, 1, 2, 2, 2], vec![1, 2]),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(majority_element_ii(nums), expected);
        }
    }
}
