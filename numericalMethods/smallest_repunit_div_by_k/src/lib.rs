/*
Given a positive integer K, you need to find the length of the smallest positive integer N such
that N is divisible by K, and N only contains the digit 1. (eg, 1, 11, 111...)

Return the length of N. If there is no such N, return -1.

Constraints:

    1 <= K <= 10^5

*/

pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
    if k == 1 {
        return 1;
    }
    if k % 2 == 0 || k % 5 == 0 {
        return -1;
    }
    let mut counter = 1;
    let mut remainder = 1;
    while remainder != 0 {
        remainder *= 10;
        remainder += 1;
        remainder %= k;
        counter += 1;
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (1, 1),
            (2, -1),
            (3, 3),
            (4, -1),
            (5, -1),
            (7, 6),
            (9, 9),
            (11, 2),
            (13, 6),
            (27027, 54),
            (99991, 49995),
        ];
        for (k, expected) in test_tuples {
            assert_eq!(smallest_repunit_div_by_k(k), expected);
        }
    }
}
