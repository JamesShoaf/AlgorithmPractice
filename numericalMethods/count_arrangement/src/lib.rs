/*
Suppose you have n integers from 1 to n. We define a beautiful arrangement as an array that is
constructed by these n numbers successfully if one of the following is true for the ith
position (1 <= i <= n) in this array:

    The number at the ith position is divisible by i.
    i is divisible by the number at the ith position.

Given an integer n, return the number of the beautiful arrangements that you can construct.
*/

fn backtrack(i: usize, res: &mut i32, included: &mut Vec<bool>) {
    if i == included.len() {
        *res += 1;
        return;
    }
    for n in 1..included.len() {
        if !included[n] && (n % i == 0 || i % n == 0) {
            included[n] = true;
            backtrack(i + 1, res, included);
            included[n] = false;
        }
    }
}

pub fn count_arrangement(n: i32) -> i32 {
    let mut included: Vec<bool> = vec![false; n as usize + 1];
    included[0] = true;
    let mut res = 0;
    backtrack(1, &mut res, &mut included);
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 8),
            (5, 10),
            (6, 36),
            (7, 41),
            (8, 132),
            (15, 24679),
        ];
        for (n, expected) in test_tuples {
            assert_eq!(super::count_arrangement(n), expected);
        }
    }
}
