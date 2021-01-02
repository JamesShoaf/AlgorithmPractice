/* 
Given an array of positive integers arr,  find a pattern of length m that is repeated k or more times.

A pattern is a subarray (consecutive sub-sequence) that consists of one or more values, repeated multiple times consecutively without overlapping. A pattern is defined by its length and the number of repetitions.

Return true if there exists a pattern of length m that is repeated k or more times, otherwise return false.
*/

fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
    if m * k > arr.len() as i32 { return false; }
    let m = m as usize;
    let mut sequential = vec![1; m];
    for i in m..=arr.len() - m {
        let mut full_match = true;
        for j in 0..m {
            if arr[i + j - m] != arr[i + j] {
                full_match = false;
                break;
            }
        }
        if full_match { sequential[i % m] += 1 } else { sequential[i % m] = 1 };
        if sequential[i % m] >= k { return true; }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), 1, 1, false),
            (vec![1, 2], 1, 1, true),
            (vec![1, 2], 3, 1, false),
            (vec![1, 2, 4, 4, 4, 4], 1, 3, true),
            (vec![1, 2, 1, 2, 1, 1, 1, 3], 2, 2, true),
            (vec![1, 2, 1, 2, 1, 3], 2, 3, false),
            (vec![1, 2, 3, 1, 2], 2, 2, false),
            (vec![2, 2, 2, 2], 2, 3, false),
        ];

        for (arr, m, k, expected) in test_tuples {
            assert_eq!(contains_pattern(arr, m, k), expected);
        }
    }
}
