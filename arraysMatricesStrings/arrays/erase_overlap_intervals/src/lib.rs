/* Given a collection of intervals, find the minimum number of intervals you need to remove to make
the rest of the intervals non-overlapping. 

    You may assume the interval's end point is always bigger than its start point.
    Intervals like [1,2] and [2,3] have borders "touching" but they don't overlap each other.

*/

fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    let len = intervals.len();
    if len == 0 { return 0; }
    intervals.sort();
    let mut dp = vec![len; len];
    dp[len - 1] = 0;
    for i in (0..len - 1).rev() {
        for j in i + 1..len {
            if intervals[i][1] <= intervals[j][0] {
                dp[i] = std::cmp::min(dp[i], j - 1 - i + dp[j]);
            }
        }
        if dp[i] == len {dp[i] -= i + 1; }
    }
    dp[0] as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![vec![1, 2]], 0),
            (vec![vec![1, 2],vec![2, 3],vec![3, 4],vec![1, 3]], 1),
            (vec![vec![1, 2],vec![1, 2],vec![1, 2]], 2),
            (vec![vec![1, 2],vec![2, 3]], 0),
            (vec![vec![3, 5],vec![2, 5],vec![1, 2],vec![1, 3],vec![2, 4],vec![2, 3]], 3),
        ];
        for (vec, expected) in test_tuples {
            assert_eq!(super::erase_overlap_intervals(vec), expected);
        }
    }
}
