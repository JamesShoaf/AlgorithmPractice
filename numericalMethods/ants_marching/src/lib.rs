// The ants go marching 2x2 in the first verse, followed by 3x3, 4x4...
// Assuming that each row of ants contains exactly the same number of ants and there are
// no stragglers, what is the minimum number of ants in formation

use gcd::Gcd;

pub fn ants_marching(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut res = 1;
    for i in 2..n + 1 {
        res *= i / res.gcd(i);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ants_marching() {
        let test_tuples = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 6),
            (4, 12),
            (5, 60),
            (6, 60),
            (7, 420),
            (8, 840),
            (9, 2520),
            (10, 2520),
        ];
        for (n, expected) in test_tuples {
            assert_eq!(ants_marching(n), expected);
        }
    }
}
