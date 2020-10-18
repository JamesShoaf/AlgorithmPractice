/* 
Given a string s, return the length of the longest substring between two equal characters, excluding the two characters. If there is no such substring return -1.

A substring is a contiguous sequence of characters within a string.
*/

use std::collections::HashMap;
use std::cmp;

pub fn max_length_between_equal_characters(s: String) -> i32 {
    let mut best = -1;
    let mut first_index: HashMap<char, usize> = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        if let Some(&j) = first_index.get(&c) {
            best = cmp::max(best, (i - j) as i32 - 1);
        } else {
            first_index.insert(c, i);
        }
    }
    best
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
