/* 
Given a non-empty array of unique positive integers A, consider the following graph:

    There are A.length nodes, labelled A[0] to A[A.length - 1];
    There is an edge between A[i] and A[j] if and only if A[i] and A[j] share a common factor greater than 1.

Return the size of the largest connected component in the graph.

    1 <= A.length <= 20000
    1 <= A[i] <= 100000

*/

/* A general approach to solving this problem would be to build the graph by finding whether each
of n nodes connected to the other n-1 nodes by checking for a gcd between the nodes greater than 1.
This approach works, but building the graph takes O(n^2) time and O(n^2) space (while traversing takes
O(n) time and O(n) space).
*/
fn gcd(a: i32, b:i32) -> i32 {
    if b > a { return gcd(b, a); }
    if b <= 0 { return 0; }
    match a % b {
        0 => b,
        1 => 1,
        _ => gcd(b, a % b),
    }
}

/* 
An alternative approach is Union-Find (or Disjoint Set). This takes O(MAX) space and O(n*sqrt(MAX)) time
where MAX is the maximum value of elements in the array.
*/

fn find(n: i32, parent: &mut [i32;100001]) -> i32 {
    let n = n as usize;
    if parent[n] == -1 { return n as i32; }
    parent[n] = find(parent[n], parent);
    return parent[n];
}

fn union(x: i32, y: i32, parent: &mut [i32;100001]) -> () {
    let xp = find(x, parent);
    let yp = find(y, parent);
    if xp != yp {
        parent[yp as usize] = xp;
    }
}

fn largest_component_size(a: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    use std::cmp::max;
    let mut parent: [i32; 100001] = [-1; 100001];
    for n in a.iter() {
        let n = *n;
        for i in 2..=(n as f64).sqrt() as i32 {
            if n % i == 0 {
                union(i, n, &mut parent);
                union(n, n/i, &mut parent);
            }
        }
    }
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut count = 0;
    for n in a {
        let np = find(n, &mut parent);
        let np_count = map.entry(np).or_insert(0);
        *np_count += 1;
        count = max(count, *np_count);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        let test_tuples = vec![
            (252, 105, 21),
            (12, 39, 3),
            (5, 7, 1),
            (-8, 2, 0),
            (0, 12, 0),
        ];
        for (a, b, expected) in test_tuples {
            assert_eq!(gcd(a, b), expected);
        }
    }

    #[test]
    fn test_largest_component_size() {
        let test_tuples = vec![
            (vec![3, 9, 2, 4, 6], 5),
            (vec![4, 6, 15, 35], 4),
            (vec![20, 50, 9, 63], 2),
            (vec![2, 3, 6, 7, 4, 12, 21, 39], 8),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(largest_component_size(nums), expected);
        }
    }
}
