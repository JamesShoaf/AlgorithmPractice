/*
Given four lists A, B, C, D of integer values, compute how many tuples (i, j, k, l) there are such
that A[i] + B[j] + C[k] + D[l] is zero.

To make problem a bit easier, all A, B, C, D have same length of N where 0 ≤ N ≤ 500. All integers
are in the range of -2^28 to 2^28 - 1 and the result is guaranteed to be at most 2^31 - 1.
*/

use std::collections::HashMap;

pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
    let mut ab_sums = HashMap::new();
    for i in 0..a.len() {
        for j in 0..b.len() {
            *ab_sums.entry(a[i] + b[j]).or_insert(0) += 1;
        }
    }
    let mut res = 0;
    for i in 0..c.len() {
        for j in 0..d.len() {
            if let Some(&count) = ab_sums.get(&(-c[i] - d[j])) {
                res += count;
            }
        }
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
