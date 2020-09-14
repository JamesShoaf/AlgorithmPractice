use std::cmp;

pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    // if there are no other intervals
    if intervals.len() == 0 { return vec![new_interval]; }

    let mut output: Vec<Vec<i32>> = Vec::new();
    // if new_interval is less than all other intervals
    if new_interval[1] < intervals[0][0] {
        output.push(new_interval);
        output.append(&mut intervals);
        return output;
    }

    let mut i = 0;
    while i < intervals.len() && intervals[i][1] < new_interval[0] {
        output.push(intervals[i].clone());
        i += 1;
    }

    // if all intervals are less than new_interval
    if i == intervals.len() {
        output.push(new_interval);
        return output;
    }

    // intervals[i][1] >= new_interval[0]
    // if new_interval is between the previous interval and the current one
    // or if new_interval is a subset of the current interval
    if new_interval[1] < intervals[i][0]
        || intervals[i][0] <= new_interval[0] && intervals[i][1] >= new_interval[1] {
        if new_interval[1] < intervals[i][0] {
            output.push(new_interval);
        }
        for j in i..intervals.len() {
            output.push(intervals[j].clone());
        }
        return output;
    }

    // new_interval overlaps the previous and/or a next interval
    let mut next_interval = vec![cmp::min(intervals[i][0], new_interval[0])];
    while i < intervals.len() && intervals[i][0] <= new_interval[1] {
        i += 1;
    }
    // intervals[i][0] > new_interval[1]
    next_interval.push(cmp::max(intervals[i - 1][1], new_interval[1]));
    output.push(next_interval);
    // gather remaining intervals
    for j in i..intervals.len() {
        output.push(intervals[j].clone());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                Vec::new(),
                vec![0, 1],
                vec![vec![0, 1]],
            ),
            (
                vec![vec![2, 3], vec![6, 9]],
                vec![0, 1],
                vec![vec![0, 1], vec![2, 3], vec![6, 9]],
            ),
            (
                vec![vec![1, 3], vec![6, 9]],
                vec![2, 5],
                vec![vec![1, 5], vec![6, 9]],
            ),
            (
                vec![vec![1, 5]],
                vec![2, 3],
                vec![vec![1, 5]],
            ),
            (
                vec![vec![1, 5]],
                vec![0, 3],
                vec![vec![0, 5]],
            ),
            (
                vec![vec![1, 5]],
                vec![2, 6],
                vec![vec![1, 6]],
            ),
            (
                vec![vec![1, 3], vec![6, 9]],
                vec![4, 5],
                vec![vec![1, 3], vec![4, 5], vec![6, 9]],
            ),
            (
                vec![vec![1, 3], vec![6, 9]],
                vec![3, 6],
                vec![vec![1, 9]],
            ),
            (
                vec![vec![1, 3], vec![6, 9]],
                vec![10, 11],
                vec![vec![1, 3], vec![6, 9], vec![10, 11]],
            ),
            (
                vec![vec![1, 3], vec![6, 9]],
                vec![9, 11],
                vec![vec![1, 3], vec![6, 11]],
            ),
            (
                vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], 
                vec![4, 8],
                vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            ),
        ];
        for (intervals, new_interval, expected) in test_tuples {
            assert_eq!(insert(intervals, new_interval), expected);
        }
    }
}
