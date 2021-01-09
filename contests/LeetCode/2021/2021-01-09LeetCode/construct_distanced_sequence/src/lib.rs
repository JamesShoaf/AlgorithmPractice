/*
Given an integer n, find a sequence that satisfies all of the following:

    The integer 1 occurs once in the sequence.
    Each integer between 2 and n occurs twice in the sequence.
    For every integer i between 2 and n, the distance between the two occurrences of i is exactly i.

The distance between two numbers on the sequence, a[i] and a[j], is the absolute difference of their
indices, |j - i|.

Return the lexicographically largest sequence. It is guaranteed that under the given constraints,
there is always a solution.

A sequence a is lexicographically larger than a sequence b (of the same length) if in the first
position where a and b differ, sequence a has a number greater than the corresponding number in b.
For example, [0,1,9,0] is lexicographically larger than [0,1,5,6] because the first position they
differ is at the third number, and 9 is greater than 5.
*/

use std::collections::HashSet;

fn backtrack(n: i32, res: &mut Vec<i32>, included: &mut HashSet<i32>) -> bool {
    if included.len() == n as usize {
        return true;
    }
    let mut i = 0;
    while res[i] != 0 {
        i += 1;
    }
    for k in (2..n + 1).rev() {
        let j = i + k as usize;
        if j < res.len() && res[j] == 0 && !included.contains(&k) {
            res[i] = k;
            res[j] = k;
            included.insert(k);
            if backtrack(n, res, included) {
                return true;
            }
            res[i] = 0;
            res[j] = 0;
            included.remove(&k);
        }
    }
    if !included.contains(&1) {
        res[i] = 1;
        included.insert(1);
        if backtrack(n, res, included) {
            return true;
        }
        res[i] = 0;
        included.remove(&1);
    }
    false
}

pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    if n < 1 {
        return Vec::new();
    }
    let mut res = vec![0; 2 * n as usize - 1];
    let mut included = HashSet::new();
    backtrack(n, &mut res, &mut included);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![(1, vec![1]), (2, vec![2, 1, 2]), (3, vec![3, 1, 2, 3, 2])];
        for (n, expected) in test_tuples {
            assert_eq!(construct_distanced_sequence(n), expected);
        }
    }
}
