/*
Given a positive integer n, find the smallest integer which has exactly the same digits existing in
the integer n and is greater in value than n. If no such positive integer exists, return -1.
*/

use std::convert::TryFrom;

pub fn next_greater_element(n: i32) -> i32 {
    if n < 10 {
        return -1;
    }
    let mut n = n as usize;
    let mut current_power = 1;
    while n / current_power >= 10 {
        current_power *= 10;
    }
    let mut digit_stack = Vec::new();
    while current_power > 0 {
        let current_digit = n / current_power;
        digit_stack.push(current_digit);
        n -= current_digit * current_power;
        current_power /= 10;
    }
    let mut digit_count = [0; 10];
    while let Some(digit) = digit_stack.pop() {
        digit_count[digit] += 1;
        if let Some(higher) = (digit + 1..=9).find(|&i| digit_count[i] > 0) {
            digit_count[higher] -= 1;
            digit_stack.push(higher);
            for i in 0..=9 {
                for _ in 0..digit_count[i] {
                    digit_stack.push(i);
                }
            }
            break;
        }
    }
    if digit_stack.is_empty() {
        return -1;
    }
    let mut res: usize = 0;
    for i in 0..digit_stack.len() {
        if let Some(product) = res.checked_mul(10) {
            res = product;
        } else {
            return -1;
        }
        if let Some(sum) = res.checked_add(digit_stack[i]) {
            res = sum;
        } else {
            return -1;
        }
    }
    if let Ok(int) = i32::try_from(res) {
        return int;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (12, 21),
            (21, -1),
            (1111111112, 1111111121),
            (1211111111, 2111111111),
            (2111111111, -1),
            (1111111119, 1111111191),
            (1911111111, -1),
        ];
        for (n, expected) in test_tuples {
            assert_eq!(next_greater_element(n), expected);
        }
    }
}
