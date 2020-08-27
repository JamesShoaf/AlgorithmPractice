use std::collections::HashMap;

fn find_right_interval(mut intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let mut output: Vec<i32> = vec![-1; intervals.len()];
    let mut map: HashMap<Vec<i32>, i32> = HashMap::new();
    for (i, interval) in intervals.iter().enumerate() {
        map.insert(interval.clone(), i as i32);
    }
    intervals.sort();
    for (i, interval) in intervals.iter().enumerate() {
        let target = interval[1];
        let mut low = i;
        let mut high = intervals.len();
        while low < high {
            let mid = (low + high) / 2;
            let mid_val = intervals[mid][0];
            if mid_val < target { low = mid + 1; }
            else { high = mid; }
        }
        if low < intervals.len() {
            if let Some(original_index) = map.get(interval) {
                if let Some(right_index) = map.get(&intervals[low]) {
                    output[*original_index as usize] = *right_index;
                }
            }
        }
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
                vec![
                    vec![1, 2],
                ], vec![-1],
            ),
            (
                vec![
                    vec![3, 4],
                    vec![2, 3],
                    vec![1, 2],
                ], vec![-1, 0, 1],
            ),
            (
                vec![
                    vec![1, 4],
                    vec![2, 3],
                    vec![3, 4],
                ], vec![-1, 2, -1],
            ),
            (
                vec![
                    vec![4, 5],
                    vec![3, 5],
                    vec![2, 5],
                    vec![1, 5],
                    vec![5, 6],
                ], vec![4, 4, 4, 4, -1],
            ),
            (
                Vec::new(),
                Vec::new(),
            ),
        ];
        for (intervals, expected) in test_tuples {
            assert_eq!(find_right_interval(intervals), expected);
        }
    }
}
