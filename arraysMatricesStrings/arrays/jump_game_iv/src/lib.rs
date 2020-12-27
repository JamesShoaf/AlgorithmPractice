/*
Given an array of integers arr, you are initially positioned at the first index of the array.

In one step you can jump from index i to index:

    i + 1 where: i + 1 < arr.length.
    i - 1 where: i - 1 >= 0.
    j where: arr[i] == arr[j] and i != j.

Return the minimum number of steps to reach the last index of the array.

Notice that you can not jump outside of the array at any time.
*/

use std::collections::HashMap;

pub fn min_jumps(arr: Vec<i32>) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    let mut value_map: HashMap<i32, Vec<usize>> = HashMap::new();
    for i in 0..arr.len() {
        value_map.entry(arr[i]).or_insert(Vec::new()).push(i);
    }
    let mut queued_indices = vec![false; arr.len()];
    let mut level = vec![0];
    queued_indices[0] = true;
    let mut counter = 0;
    loop {
        let mut next_level = Vec::new();
        for i in level {
            if i == arr.len() - 1 {
                return counter;
            }
            if arr.len() > i + 1 && !queued_indices[i + 1] {
                queued_indices[i + 1] = true;
                next_level.push(i + 1);
            }
            if i > 0 && !queued_indices[i - 1] {
                queued_indices[i - 1] = true;
                next_level.push(i - 1);
            }
            if let Some(portals) = value_map.get(&arr[i]) {
                for &j in portals.iter() {
                    if queued_indices[j] == false {
                        queued_indices[j] = true;
                        next_level.push(j);
                    }
                }
                value_map.remove(&arr[i]);
            }
        }
        level = next_level;
        counter += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404], 3),
            (vec![7], 0),
            (vec![7, 6, 9, 6, 9, 6, 9, 7], 1),
            (vec![6, 1, 9], 2),
            (vec![11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13], 3),
        ];
        for (arr, expected) in test_tuples {
            assert_eq!(super::min_jumps(arr), expected);
        }
    }
}
