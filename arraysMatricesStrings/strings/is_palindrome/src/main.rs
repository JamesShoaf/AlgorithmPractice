struct Solution {}

// Given a string, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s.chars()
            .filter(|chr| chr.is_ascii_alphanumeric())
            .map(|chr| chr.to_ascii_uppercase());
        for (backward, forward) in chars.clone().rev().zip(chars) {
            if forward != backward { return false; }
        }
        true
    }
}

fn main() {
    let test_tuples = vec![
        (String::from(""), true),
        (String::from("L😂O😆L🤣"), true),
        (String::from("a"), true),
        (String::from("aa"), true),
        (String::from("ab"), false),
        (String::from("A man, a plan, a canal: Panama"), true),
        (String::from("A man, plan, and canal: Panama"), false),
        (String::from("Racecar"), true),
        (String::from("race Car"), true),
        (String::from("race a car"), false),
        (String::from("0P"), false),
        (String::from("1O1O1"), true),
    ];
    for (string, expected) in test_tuples {
        assert_eq!(Solution::is_palindrome(string), expected);
    }
}
