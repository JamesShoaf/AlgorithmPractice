/*
A message containing letters from A-Z is being encoded to numbers using the following mapping:

    'A' -> 1
    'B' -> 2
    ...
    'Z' -> 26

Given a non-empty string containing only digits, determine the total number of ways to decode it.

The answer is guaranteed to fit in a 32-bit integer.
*/

pub fn num_decodings(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() || chars[0] == '0' {
        return 0;
    }
    let mut dp = vec![0; chars.len() + 2];
    let len = dp.len();
    dp[len - 2] = 1;
    for i in (0..dp.len() - 2).rev() {
        let c = chars[i];
        dp[i] = match c {
            '1' => dp[i + 1] + dp[i + 2],
            '2' => {
                dp[i + 1]
                    + if i + 1 < chars.len() {
                        match chars[i + 1] {
                            '0' | '1' | '2' | '3' | '4' | '5' | '6' => dp[i + 2],
                            _ => 0,
                        }
                    } else {
                        0
                    }
            }
            '0' => 0,
            _ if c.is_digit(10) => dp[i + 1],
            _ => return 0,
        }
    }
    *dp.first().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_tuples = vec![
            ("12".to_string(), 2),
            ("226".to_string(), 3),
            ("0".to_string(), 0),
            ("1".to_string(), 1),
            ("00000000000000001".to_string(), 0),
            ("01001".to_string(), 0),
            ("00012340".to_string(), 0),
            ("000123410".to_string(), 0),
            ("202112320".to_string(), 8),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(super::num_decodings(s), expected);
        }
    }
}
