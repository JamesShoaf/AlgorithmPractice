/* 
Given a list of intervals, remove all intervals that are covered by another interval in the list.

Interval [a,b) is covered by interval [c,d) if and only if c <= a and b <= d.

After doing so, return the number of remaining intervals.
*/
use std::collections::HashMap;

pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort();
    let mut map: HashMap<i32, i32> = HashMap::new();
    for interval in intervals.iter() {
        map.insert(interval[0], interval[1]);
    }
    let mut max_end = i32::MIN;
    let mut uncovered_intervals = 0;
    for interval in intervals {
        if let Some(end) = map.get(&interval[0]) {
            if *end > max_end {
                max_end = *end;
                uncovered_intervals += 1;
            }
        }
    }
    uncovered_intervals
}

// #1 solution for comparison

// use std::cmp::Ordering;

// pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
//     intervals.sort_by(|first, second| {
//         let starting = first[0].cmp(&second[0]);
        
//         if let Ordering::Equal = starting {
//             let firstLength = interval_len(first);
//             let secondLength = interval_len(second);
//             return secondLength.cmp(&firstLength);
//         } 
        
//         starting
//     });
            
//     intervals.into_iter().fold(Vec::<Vec<i32>>::new(), |mut intervals, interval| {
//         if intervals.is_empty() {
//             intervals.push(interval);
//             return intervals;
//         }
        
//         let prev = &intervals[intervals.len() - 1];
        
//         // Contained in previous interval
//         if prev[0] <= interval[0] && prev[1] >= interval[1] {
//             return intervals;
//         }
        
//         intervals.push(interval);
        
//         intervals
//     }).len() as i32
// }

// fn interval_len(interval: &[i32]) -> i32 {
//     interval[1] - interval[0]    
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![
                vec![1, 4],
                vec![3, 6],
                vec![2, 8],
            ],
            2),
            (vec![
                vec![1, 4],
                vec![2, 3],
            ],
            1),
            (vec![
                vec![0, 10],
                vec![5, 12],
            ],
            2),
            (vec![
                vec![3, 10],
                vec![4, 10],
                vec![5, 11],
            ],
            2),
            (vec![
                vec![1, 2],
                vec![1, 4],
                vec![3, 4],
            ],
            1),
            (Vec::new(),
            0),
        ];
        for (intervals, expected) in test_tuples {
            assert_eq!(remove_covered_intervals(intervals), expected);
        }
    }
}
