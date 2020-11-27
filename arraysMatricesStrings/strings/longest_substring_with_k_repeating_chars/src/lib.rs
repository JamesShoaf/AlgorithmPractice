/*
Given a string s and an integer k, return the length of the longest substring of s such that the
frequency of each character in this substring is greater than or equal to k.
*/

use std::cmp;
use std::collections::{HashMap, HashSet};

fn get_char_counts(s: &[char]) -> HashMap<char, usize> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for &c in s {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

fn get_letters_with_count_less_than_k(counts: HashMap<char, usize>, k: usize) -> HashSet<char> {
    counts
        .into_iter()
        .filter(|&(_, count)| count < k)
        .map(|(c, _)| c)
        .collect()
}

fn recursive_helper(s: &[char], k: usize, mut best: usize) -> usize {
    // base case - if s is too short to contain the best possible substring (such as if it's empty)
    if s.len() <= best {
        return 0;
    }
    let counts = get_char_counts(s);
    let letters = get_letters_with_count_less_than_k(counts, k);
    if letters.len() == 0 {
        return s.len();
    }
    let mut start = 0;
    for i in 0..s.len() {
        if letters.contains(&s[i]) {
            best = cmp::max(best, recursive_helper(&s[start..i], k, best));
            start = i + 1;
        }
    }
    cmp::max(best, recursive_helper(&s[start..s.len()], k, best))
}

pub fn longest_substring(s: String, k: i32) -> i32 {
    let k = k as usize;
    let s: Vec<char> = s.chars().collect();
    recursive_helper(&s, k, 0) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("aaa"), 4, 0),
            (String::from("aaabb"), 3, 3),
            (String::from("ababbc"), 2, 5),
            (String::from("aaacababdbb"), 2, 4),
            (String::from("aaaaaaaaaaaaaaaaaaaa"), 100, 0),
            (String::from("aabcccdeeeee"), 2, 5),
        ];
        for (s, k, expected) in test_tuples {
            assert_eq!(longest_substring(s, k), expected);
        }
    }
}
