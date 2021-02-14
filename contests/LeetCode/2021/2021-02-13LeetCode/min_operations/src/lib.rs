/*
You are given a string s consisting only of the characters '0' and '1'.
In one operation, you can change any '0' to '1' or vice versa.

The string is called alternating if no two adjacent characters are equal.
For example, the string "010" is alternating, while the string "0100" is not.

Return the minimum number of operations needed to make s alternating.
*/

use std::cmp;

pub fn min_operations(s: String) -> i32 {
    let mut even_ones = 0;
    let mut odd_ones = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '1' {
            if i % 2 == 0 {
                even_ones += 1;
            } else {
                odd_ones += 1;
            }
        }
    }
    cmp::min(
        odd_ones + (s.len() + 1) / 2 - even_ones,
        even_ones + s.len() / 2 - odd_ones,
    ) as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
