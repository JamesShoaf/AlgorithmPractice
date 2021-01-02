/*
Given an integer n, return the decimal value of the binary string formed by concatenating the
binary representations of 1 to n in order, modulo 10^9 + 7.
*/

fn most_significant_set_bit(mut n: i32) -> u32 {
    let mut res: u32 = 0;
    while n > 0 {
        res += 1;
        n >>= 1;
    }
    res
}

pub fn concatenated_binary(n: i32) -> i32 {
    let modulus: i32 = 10_i32.pow(9) + 7;
    let mut res = 0;
    for i in 1..=n {
        for _ in 0..most_significant_set_bit(i) {
            res *= 2;
            res %= modulus;
        }
        res += i;
        res %= modulus;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![(1, 1), (2, 6), (3, 27), (12, 505379714)];
        for (n, expected) in test_tuples {
            assert_eq!(concatenated_binary(n), expected);
        }
    }
}
