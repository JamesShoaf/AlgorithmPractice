/*
Given a string s, return the number of homogenous substrings of s.
Since the answer may be too large, return it modulo 109 + 7.

A string is homogenous if all the characters of the string are the same.

A substring is a contiguous sequence of characters within a string.
*/

const MODULUS: i32 = 10_i32.pow(9) + 7;

pub fn count_homogenous(s: String) -> i32 {
    let mut last_char = 'a';
    let mut count = 0;
    let mut res = 0;
    for c in s.chars() {
        if last_char == c {
            count += 1;
        } else {
            count = 1;
            last_char = c;
        }
        res += count;
        res %= MODULUS;
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
