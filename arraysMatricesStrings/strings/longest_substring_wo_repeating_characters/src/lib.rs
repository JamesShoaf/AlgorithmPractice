use std::cmp;
use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut longest = 0;
    let mut l = 0;
    let mut set: HashSet<char> = HashSet::new();
    for r in 0..chars.len() {
        while set.contains(&chars[r]) {
            set.remove(&chars[l]);
            l += 1;
        }
        set.insert(chars[r]);
        longest = cmp::max(longest, set.len());
    }
    longest as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_tuples = vec![
            ("abcabcbb".to_string(), 3),
            ("bbbbb".to_string(), 1),
            ("pwwkew".to_string(), 3),
            ("".to_string(), 0),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(super::length_of_longest_substring(s), expected);
        }
    }
}
