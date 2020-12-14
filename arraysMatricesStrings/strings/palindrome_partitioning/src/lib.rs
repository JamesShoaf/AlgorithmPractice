/*
Given a string s, partition s such that every substring of the partition is a palindrome. Return
all possible palindrome partitioning of s.

A palindrome string is a string that reads the same backward as forward.
*/

fn is_palindrome(s: &Vec<char>) -> bool {
    if s.len() < 2 {
        return true;
    }
    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }
    true
}

// backtracking solution - O(n^2^n) time
fn partition_helper(
    start: usize,
    chars: &Vec<char>,
    res: &mut Vec<Vec<String>>,
    prefix: &mut Vec<String>,
) {
    if start == chars.len() {
        return res.push(prefix.clone());
    }
    let mut current = Vec::new();
    for i in start..chars.len() {
        current.push(chars[i]);
        if is_palindrome(&current) {
            prefix.push(current.iter().collect());
            partition_helper(i + 1, chars, res, prefix);
            prefix.pop();
        }
    }
}

pub fn partition(s: String) -> Vec<Vec<String>> {
    let chars = s.chars().collect();
    let mut res = Vec::new();
    partition_helper(0, &chars, &mut res, &mut vec![]);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                String::from("aab"),
                vec![
                    vec![String::from("a"), String::from("a"), String::from("b")],
                    vec![String::from("aa"), String::from("b")],
                ],
            ),
            (String::from("a"), vec![vec![String::from("a")]]),
            (
                String::from("aabba"),
                vec![
                    vec![
                        String::from("a"),
                        String::from("a"),
                        String::from("b"),
                        String::from("b"),
                        String::from("a"),
                    ],
                    vec![
                        String::from("a"),
                        String::from("a"),
                        String::from("bb"),
                        String::from("a"),
                    ],
                    vec![String::from("a"), String::from("abba")],
                    vec![
                        String::from("aa"),
                        String::from("b"),
                        String::from("b"),
                        String::from("a"),
                    ],
                    vec![String::from("aa"), String::from("bb"), String::from("a")],
                ],
            ),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(partition(s), expected);
        }
    }
}
