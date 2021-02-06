/*
You are given an array of events where events[i] = [startDayi, endDayi, valuei].
The ith event starts at startDayi and ends at endDayi, and if you attend this event,
you will receive a value of valuei. You are also given an integer k which represents
the maximum number of events you can attend.

You can only attend one event at a time. If you choose to attend an event, you must
attend the entire event. Note that the end day is inclusive: that is, you cannot attend
two events where one of them starts and the other ends on the same day.

Return the maximum sum of values that you can receive by attending events.
*/

use std::cmp;
use std::collections::HashMap;

// memoized recursive solution - O(max(logn, k)*n) time and O(n*k) space
fn recursive(
    i: usize,
    rem: i32,
    time: i32,
    events: &Vec<Vec<i32>>,
    memo: &mut HashMap<(usize, i32, i32), i32>,
    max_start: i32,
) -> i32 {
    if i == events.len() || rem < 1 || time > max_start {
        return 0;
    }
    if let Some(&prev) = memo.get(&(i, rem, time)) {
        return prev;
    }
    let (start, end, val) = (events[i][0], events[i][1], events[i][2]);
    let mut res = recursive(i + 1, rem, time, events, memo, max_start);
    if start > time {
        res = cmp::max(
            res,
            val + recursive(i + 1, rem - 1, end, events, memo, max_start),
        );
    }
    memo.insert((i, rem, time), res);
    res
}

pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
    events.sort_unstable();
    // index, remaining_events, start
    let mut memo: HashMap<(usize, i32, i32), i32> = HashMap::new();
    let max_start = events.iter().map(|vec| vec[0]).max().unwrap_or(0);
    recursive(0, k, 0, &events, &mut memo, max_start)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
