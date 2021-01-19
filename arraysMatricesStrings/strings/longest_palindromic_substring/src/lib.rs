/*
Given a string s, return the longest palindromic substring in s.
*/

pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return s;
    }
    let chars = s.chars().collect::<Vec<char>>();
    let mut best_start = 0;
    let mut best_end = 0;
    for i in 0..chars.len() {
        let mut start = i;
        let mut end = i;
        while end < chars.len() - 1 && chars[end + 1] == chars[i] {
            end += 1;
        }
        while start > 0 && end < chars.len() - 1 && chars[start - 1] == chars[end + 1] {
            start -= 1;
            end += 1;
        }
        if end - start >= best_end - best_start {
            best_start = start;
            best_end = end;
        }
    }
    (best_start..best_end + 1).map(|i| chars[i]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            ("babad".to_string(), 3),
            ("cbbd".to_string(), 2),
            ("cbbbd".to_string(), 3),
            ("cbbbc".to_string(), 5),
            ("a".to_string(), 1),
            ("ac".to_string(), 1),
        ];
        for (s, len) in test_tuples {
            let res = longest_palindrome(s);
            let rev = res.chars().rev().collect::<String>();
            assert_eq!(res, rev);
            assert_eq!(res.len(), len);
        }
    }
}
