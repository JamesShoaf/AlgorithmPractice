/*
Given a string s. An awesome substring is a non-empty substring of s such that we can make any number of swaps in order to make it palindrome.

Return the length of the maximum length awesome substring of s.
*/

struct Solution {}

use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut longest = 1;
        // initial state - all 10 bits turned off - selection is arbitrary, could also choose 1023 or 7
        let mut current = 0;
        // store initial state at index 0 - for cases where all bits are switched off again
        map.insert(current, 0);

        for (index, num) in s.chars().enumerate() {
            // toggle the bit in the state corresponding with the character
            current ^= 1 << num.to_digit(10).unwrap();
            // if the state matches a previous state, compare the length of this match with the longest match
            longest = cmp::max(longest, 1 + index - *map.entry(current).or_insert(index + 1));
            // palindromes are allowed 1 unpaired character, so also match with states that are 1 bit off
            for digit in 0..10 {
                if let Some(prev_index) = map.get(&(current ^ (1 << digit))) {
                    longest = cmp::max(longest, 1 + index - *prev_index);
                }
            }
        }
        // return the longest match
        longest as i32
    }
}

fn main() {
    // Solution::longest_awesome(String::from("00"));
    let test_tuples = vec![
        (String::from("3242415"), 5),
        (String::from("12345678"), 1),
        (String::from("213123"), 6),
        (String::from("00"), 2),
    ];
    for (s, expected) in test_tuples {
        assert_eq!(Solution::longest_awesome(s), expected);
    }
}
