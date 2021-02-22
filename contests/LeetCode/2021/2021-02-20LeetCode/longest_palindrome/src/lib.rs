/*
You are given two strings, word1 and word2. You want to construct a string in the following manner:

    Choose some non-empty subsequence subsequence1 from word1.
    Choose some non-empty subsequence subsequence2 from word2.
    Concatenate the subsequences: subsequence1 + subsequence2, to make the string.

Return the length of the longest palindrome that can be constructed in the described manner. If no
palindromes can be constructed, return 0.

A subsequence of a string s is a string that can be made by deleting some (possibly none) characters
from s without changing the order of the remaining characters.

A palindrome is a string that reads the same forward as well as backward.
*/

use std::cmp;
use std::collections::HashMap;

fn helper(concat: &Vec<char>) -> HashMap<(usize, usize), usize> {
    let mut res = HashMap::new();
    for i in (0..concat.len()).rev() {
        res.insert((i, i), 1);
        for j in i + 1..concat.len() {
            res.insert(
                (i, j),
                if concat[i] == concat[j] {
                    res.get(&(i + 1, j - 1)).unwrap_or(&0) + 2
                } else {
                    cmp::max(
                        *res.get(&(i + 1, j)).unwrap(),
                        *res.get(&(i, j - 1)).unwrap(),
                    )
                },
            );
        }
    }
    res
}

pub fn longest_palindrome(word1: String, word2: String) -> i32 {
    let w1: Vec<char> = word1.chars().collect();
    let w2: Vec<char> = word2.chars().collect();
    let dp = helper(&w1.iter().chain(w2.iter()).map(|&c| c).collect());
    let mut res = 0;
    for i in 0..w1.len() {
        for j in 0..w2.len() {
            if w1[i] == w2[j] {
                res = cmp::max(res, 2 + dp.get(&(i + 1, w1.len() + j - 1)).unwrap_or(&0));
            }
        }
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            ("cacb".to_string(), "cbba".to_string(), 5),
            ("ab".to_string(), "ab".to_string(), 3),
            ("aa".to_string(), "bb".to_string(), 0),
        ];
        for (word1, word2, expected) in test_tuples {
            assert_eq!(longest_palindrome(word1, word2), expected);
        }
    }
}
