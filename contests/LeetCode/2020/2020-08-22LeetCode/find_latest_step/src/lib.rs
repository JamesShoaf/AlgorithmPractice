/* 
Given an array arr that represents a permutation of numbers from 1 to n. You have a binary string
of size n that initially has all its bits set to zero.

At each step i (assuming both the binary string and arr are 1-indexed) from 1 to n, the bit at
position arr[i] is set to 1. You are given an integer m and you need to find the latest step at
which there exists a group of ones of length m. A group of ones is a contiguous substring of 1s
such that it cannot be extended in either direction.

Return the latest step at which there exists a group of ones of length exactly m. If no such group
exists, return -1.
*/

fn dfs(l: i32, r: i32, arr: &Vec<i32>, i: i32, m: i32) -> i32 {
    let size = r - l + 1;
    if size == m { return i; }
    if size < m || i == -1 { return -1; }
    for j in (0..=i).rev() {
        let curr = arr[j as usize];
        if curr == l { return dfs(l + 1, r, arr, j - 1, m); }
        if curr == r { return dfs(l, r - 1, arr, j - 1, m); }
        if curr > l && curr < r {
            return std::cmp::max(dfs(l, curr - 1, arr, j - 1, m), dfs(curr + 1, r, arr, j - 1, m));
        }
    }
    -1
}

fn find_latest_step(mut arr: Vec<i32>, m: i32) -> i32 {
    if m < 1 { return -1; }
    arr.reverse();
    let len = arr.len() as i32;
    if m == len { return m; }
    dfs(1, len, &arr, len - 1, m)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![3, 5, 1, 2, 4], 1, 4),
            (vec![3, 1, 5, 2, 4], 2, -1),
        ];
        for (arr, m, expected) in test_tuples {
            assert_eq!(find_latest_step(arr, m), expected);
        }
    }
}
