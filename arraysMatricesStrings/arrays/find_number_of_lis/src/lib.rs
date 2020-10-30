/* 
Given an integer array nums, return the number of longest increasing subsequences.

These subsequences do not have to be contiguous, but they must be strictly increasing.
*/

pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
    let mut longest_subsequence: usize = 1;
    // (length, count) of longest subsequence including the number at the current index
    // all nums are initially part of a subsequence of length 1 containing only themselves
    let mut subsq_len_counts: Vec<(usize, usize)> = vec![(1, 1); nums.len()];
    // iterating backwards and scanning forwards
    for i in (0..nums.len()).rev() {
        for j in i + 1..nums.len() {
            // if the number ahead could form an increasing subsequence
            if nums[j] > nums[i] {
                // if the increasing number's subsequence is 1 less than the current length,
                // it is also a valid postfix for the current number, so add its count
                if subsq_len_counts[j].0 == subsq_len_counts[i].0 - 1 {
                    subsq_len_counts[i].1 += subsq_len_counts[j].1;
                }
                // if the subsequence would be of greater length than the current longest
                // replace the subsequence with the new length + 1 (adding num[i] to the front)
                if subsq_len_counts[j].0 >= subsq_len_counts[i].0 {
                    subsq_len_counts[i] = (subsq_len_counts[j].0 + 1, subsq_len_counts[j].1);
                }
            }
        }
        // update the longest subsequence for the final step
        if subsq_len_counts[i].0 > longest_subsequence {
            longest_subsequence = subsq_len_counts[i].0;
        }
    }
    // add up all the counts of subsequences of longest length
    subsq_len_counts.into_iter()
        .filter(|&(len, _)| len == longest_subsequence)
        .map(|(_, count)| count)
        .sum::<usize>() as i32
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
