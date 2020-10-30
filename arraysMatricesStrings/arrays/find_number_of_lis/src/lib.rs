/* 
Given an integer array nums, return the number of longest increasing subsequences.

These subsequences do not have to be contiguous, but they must be strictly increasing.
*/

pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
    // all nums are initially part of a subsequence of length 1 containing only themselves
    let mut subsq_lengths = vec![1; nums.len()];
    let mut subsq_counts = vec![1; nums.len()];
    // iterating backwards and scanning forwards
    for i in (0..nums.len()).rev() {
        for j in i + 1..nums.len() {
            // if the number ahead could form an increasing subsequence
            if nums[j] > nums[i] {
                // if the increasing number's subsequence is 1 less than the current length,
                // it is also a valid postfix for the current number, so add its count
                if subsq_lengths[j] == subsq_lengths[i] - 1 {
                    subsq_counts[i] +=  subsq_counts[j];
                }
                // if the subsequence would be of greater length than the current longest
                // replace the subsequence with the new length + 1 (adding num[i] to the front)
                if subsq_lengths[j] >= subsq_lengths[i] {
                    subsq_lengths[i] = subsq_lengths[j] + 1;
                    subsq_counts[i] = subsq_counts[j]
                }
            }
        }
    }
    let &longest_subsequence = subsq_lengths.iter().max().unwrap_or(&0);
    // add up all the counts of subsequences of longest length
    (0..nums.len())
        .filter(|&i| subsq_lengths[i] == longest_subsequence)
        .map(|i| subsq_counts[i])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), 0),
            (vec![1], 1),
            (vec![1, 3, 5, 4, 7], 2),
            (vec![1, 5, 6, 3, 4, 8], 2),
            (vec![1, 2, 3, 2, 3, 8], 3),
            (vec![1, 2, 3, 2, 3, 2, 3, 8], 6),
            (vec![2, 2, 2, 2, 2], 5),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(find_number_of_lis(nums), expected);
        }
    }
}
