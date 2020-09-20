/* 
An integer has sequential digits if and only if each digit in the number is one more than the previous digit.

Return a sorted list of all the integers in the range [low, high] inclusive that have sequential digits.
*/

fn into_sequence_length(num: i32) -> i32 {
    (num as f64).log10().floor() as i32 + 1
}

fn sequential_digits_of_length(length: i32) -> Vec<i32> {
    (1..=10 - length).map(|i| {
        (1..length).fold(i, |acc, _| 10 * acc + acc % 10 + 1)
    }).collect()
}

pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    if low < 1 || high < 1 { panic!("invalid parameter. Please enter a number between 10^2 and 10^9")}
    (into_sequence_length(low)..=into_sequence_length(high))
        .map(|l| sequential_digits_of_length(l).into_iter()
            .filter(|&x| x >= low && x <= high))
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_into_sequence_length() {
        let test_tuples = vec![
            (1, 1),
            (5, 1),
            (10, 2),
            (23, 2),
            (100, 3),
            (200, 3),
            (999, 3),
        ];
        for (num, expected) in test_tuples {
            assert_eq!(into_sequence_length(num), expected);
        }
    }

    #[test]
    fn test_sequential_digits_of_length() {
        let test_tuples = vec![
            (1, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            (2, vec![12, 23, 34, 45, 56, 67, 78, 89]),
            (3, vec![123, 234, 345, 456, 567, 678, 789]),
            (9, vec![123456789]),
            (10, Vec::new()),
        ];
        for (length, expected) in test_tuples {
            assert_eq!(sequential_digits_of_length(length), expected);
        }
    }

    #[test]
    fn test_sequential_digits() {
        let test_tuples = vec![
            (100, 120, Vec::new()),
            (100, 300, vec![123, 234]),
            (200, 500, vec![234, 345, 456]),
            (1000, 13000, vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]),
        ];
        for (low, high, expected) in test_tuples {
            assert_eq!(sequential_digits(low, high), expected);
        }
    }
}

/* 
#1 solution for comparison

use std::collections::VecDeque;

impl Solution {
    
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut answer = Vec::new();
        let mut queue = VecDeque::new();
        
        for digit in 1..=9 {
            queue.push_back(digit);
        }
        
        while !queue.is_empty() {
            let front = match queue.pop_front() {
                Some(f) => f,
                None => 0,
            };
            
            if front > high {
                break;
            }
            
            if low <= front && front <= high {
                answer.push(front);
            }
            
            let num = front % 10;
            if num < 9 {
                queue.push_back(front * 10 + (num + 1));    
            }
        }
        
        return answer;
    }
}
*/