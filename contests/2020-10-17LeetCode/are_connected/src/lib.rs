/* 
We have n cities labeled from 1 to n. Two different cities with labels x and y are directly
connected by a bidirectional road if and only if x and y share a common divisor strictly greater
than some threshold. More formally, cities with labels x and y have a road between them if there
exists an integer z such that all of the following are true:

    x % z == 0,
    y % z == 0, and
    z > threshold.

Given the two integers, n and threshold, and an array of queries, you must determine for each
queries[i] = [ai, bi] if cities ai and bi are connected (i.e. there is some path between them).

Return an array answer, where answer.length == queries.length and answer[i] is true if for the ith
query, there is a path between ai and bi, or answer[i] is false if there is no path.
*/

// Stein's Binary GCD algorithm, which did not end up being used here
pub fn gcd(a: i32, b: i32) -> i32 {
    if b > a { return gcd(b, a); }
    if a == b { return a; }
    if b == 0 { return a; }
    if a & 1 == 0 {
        if b & 1 == 0 { return gcd(a >> 1, b >> 1) << 1; }
        return gcd(a >> 1, b);
    }
    if b & 1 == 0 { return gcd(a, b >> 1); }
    gcd(a - b, b)
}

fn find(i: usize, union: &mut Vec<usize>) -> usize {
    if i == union[i] { return i; }
    union[i] = find(union[i], union);
    union[i]
}

fn unite(i: usize, j: usize, union: &mut Vec<usize>) {
    let (i_root, j_root) = (find(i, union), find(j, union));
    union[j_root] = i_root;
    find(j, union);
}

pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut union: Vec<usize> = (0..n as usize).collect();
    // modified sieve of Eratosthenes to unite all nodes with a parent higher than threshold
    for i in threshold + 1..=n {
        if find(i as usize - 1, &mut union) == i as usize - 1 {
            for j in (i * 2..=n).step_by(i as usize) {
                unite(i as usize - 1, j as usize - 1, &mut union);
            }
        }
    }
    queries.into_iter()
        .map(|vec| find(vec[0] as usize - 1, &mut union) == find(vec[1] as usize - 1, &mut union))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        let test_tuples = vec![
            (1, 1, 1),
            (1, 0, 1),
            (0, 1, 1),
            (4, 2, 2),
            (2, 4, 2),
            (8, 3, 1),
            (14, 9, 1),
            (22, 23, 1),
            (22, 25, 1),
            (12, 6, 6),
            (17, 3, 1),
            (25, 17, 1),
            (26, 14, 2),
            (4, 12, 4),
            (16, 12, 4),
            (16, 9, 1),
            (26, 3, 1),
            (20, 22, 2),
        ];
        for (a, b, expected) in test_tuples {
            assert_eq!(gcd(a, b), expected);
        }
    }
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (10000, 1, vec![vec![2, 3], vec![9967, 9973]], vec![true, false]),
            (10000, 2, vec![vec![2, 3], vec![9967, 9973]], vec![false, false]),
        ];
        for (n, threshold, queries, expected) in test_tuples {
            assert_eq!(are_connected(n, threshold, queries), expected);
        }
    }
}