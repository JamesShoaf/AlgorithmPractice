/*
Given an encoded string, return its decoded string.

The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is
being repeated exactly k times. Note that k is guaranteed to be a positive integer.

You may assume that the input string is always valid; No extra white spaces, square brackets are
well-formed, etc.

Furthermore, you may assume that the original data does not contain any digits and that digits are
only for those repeat numbers, k. For example, there won't be input like 3a or 2[4].
*/

pub fn decode_string(s: String) -> String {
    let mut substring = String::new();
    let mut result = String::new();
    let mut multiplier = 0;
    let mut bracket_depth = 0;
    for c in s.chars() {
        match c {
            '[' => {
                if bracket_depth > 0 {
                    substring.push('[');
                }
                bracket_depth += 1;
            }
            ']' => {
                bracket_depth -= 1;
                if bracket_depth > 0 {
                    substring.push(']');
                } else {
                    if !substring.is_empty() {
                        let decoded_substring = decode_string(substring);
                        for _ in 0..multiplier {
                            result.push_str(&decoded_substring);
                        }
                        substring = String::new();
                    }
                    multiplier = 0;
                }
            }
            _ if bracket_depth > 0 => substring.push(c),
            _ if c.is_digit(10) => {
                multiplier *= 10;
                multiplier += c.to_digit(10).unwrap();
            }
            _ => result.push(c),
        }
    }
    assert!(
        bracket_depth == 0,
        "invalid bracket depth - check for mismatched brackets"
    );
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("a"), String::from("a")),
            (String::from("1[a]"), String::from("a")),
            (String::from("2[a]"), String::from("aa")),
            (String::from("11[a]"), String::from("aaaaaaaaaaa")),
            (String::from("20[]"), String::from("")),
            (String::from("3[a]2[bc]"), String::from("aaabcbc")),
            (String::from("3[a2[c]]"), String::from("accaccacc")),
            (
                String::from("2[abc]3[cd]ef"),
                String::from("abcabccdcdcdef"),
            ),
            (String::from("abc3[cd]xyz"), String::from("abccdcdcdxyz")),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(decode_string(s), expected);
        }
    }
}
