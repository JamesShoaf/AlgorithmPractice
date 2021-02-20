/*
A string s is nice if, for every letter of the alphabet that s contains, it appears
both in uppercase and lowercase. For example, "abABB" is nice because 'A' and 'a'
appear, and 'B' and 'b' appear. However, "abA" is not because 'b' appears, but 'B'
does not.

Given a string s, return the longest substring of s that is nice. If there are
multiple, return the substring of the earliest occurrence. If there are none, return
an empty string.
*/

use std::collections::HashSet;

pub fn longest_nice_substring(s: String) -> String {
    let mut res = "".to_string();
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() {
        let mut uppercase: HashSet<char> = HashSet::new();
        let mut lowercase: HashSet<char> = HashSet::new();
        let mut current = String::new();
        for j in i..chars.len() {
            if !chars[j].is_ascii_alphabetic() {
                break;
            }
            current.push(chars[j]);
            if chars[j].is_lowercase() {
                lowercase.insert(chars[j].to_ascii_uppercase());
            } else {
                uppercase.insert(chars[j]);
            }
            if current.len() > res.len() && uppercase == lowercase {
                res = current.clone();
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
