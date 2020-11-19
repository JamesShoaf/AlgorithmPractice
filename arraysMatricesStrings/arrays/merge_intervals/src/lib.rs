use std::cmp;

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return intervals;
    }
    intervals.sort();
    let mut output = Vec::new();
    let mut current_start = intervals[0][0];
    let mut current_end = intervals[0][1];
    for i in 1..intervals.len() {
        if intervals[i][0] > current_end {
            output.push(vec![current_start, current_end]);
            current_start = intervals[i][0];
        }
        current_end = cmp::max(current_end, intervals[i][1]);
    }
    output.push(vec![current_start, current_end]);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
                vec![vec![1, 6], vec![8, 10], vec![15, 18]],
            ),
            (vec![vec![1, 4], vec![4, 5]], vec![vec![1, 5]]),
            (vec![vec![1, 4], vec![5, 6]], vec![vec![1, 4], vec![5, 6]]),
        ];
        for (intervals, expected) in test_tuples {
            assert_eq!(merge(intervals), expected);
        }
    }
}
