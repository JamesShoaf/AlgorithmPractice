/*
Given a parentheses string s containing only the characters '(' and ')'. A parentheses string is balanced if:

    Any left parenthesis '(' must have a corresponding two consecutive right parenthesis '))'.
    Left parenthesis '(' must go before the corresponding two consecutive right parenthesis '))'.

For example, "())", "())(())))" and "(())())))" are balanced, ")()", "()))" and "(()))" are not balanced.

You can insert the characters '(' and ')' at any position of the string to balance it if needed.

Return the minimum number of insertions needed to make s balanced.
*/

struct Solution {}

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut counter: i32 = 0;
        let mut unmatched_closed: i32 = 0;
        let mut unmatched_open: i32 = 0;
        let mut matched_pair: bool = false;
        for chr in s.chars() {
            match chr {
                '(' => {
                    // if there are any unmatched closed, close all of them, and reset the counter
                    if unmatched_closed > 0 {
                        counter += ((unmatched_closed + 1) / 2) + (unmatched_closed % 2);
                        unmatched_closed = 0;
                    }
                    // and for unfinished pairs
                    if matched_pair {
                        counter += 1;
                        matched_pair = false;
                    }
                    unmatched_open += 1;
                }
                ')' => {
                    if matched_pair {
                        matched_pair = false;
                    } else if unmatched_open > 0 {
                        unmatched_open -= 1;
                        matched_pair = true;
                    } else {
                        unmatched_closed += 1;
                    }
                }
                _ => (),
            }
        }
        if unmatched_closed > 0 { counter += ((unmatched_closed + 1) / 2) + (unmatched_closed % 2); }
        if matched_pair { counter += 1; }
        counter + unmatched_open * 2
    }
}

fn main() {
    let test_tuples = vec![
        (String::from("("), 2),
        (String::from(")"), 2),
        (String::from("))"), 1),
        (String::from("()"), 1),
        (String::from(")("), 4),
        (String::from("())"), 0),
        (String::from("()()())))"), 3),
        (String::from("(()))"), 1),
        (String::from("))())("), 3),
        (String::from("(((((("), 12),
        (String::from(")))))))"), 5),
    ];
    for (s, expected) in test_tuples {
        let output = Solution::min_insertions(s.clone());
        assert_eq!(output, expected, "expected {} insertions for {}, but got {}", expected, s, output);
    }
}
