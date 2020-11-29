/*
Given an array of non-negative integers arr, you are initially positioned at start index of the
array. When you are at index i, you can jump to i + arr[i] or i - arr[i], check if you can reach
to any index with value 0.

Notice that you can not jump outside of the array at any time.
*/

// bfs
use std::collections::VecDeque;

pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut visited = vec![false; arr.len()];
    queue.push_back(start as usize);
    while let Some(i) = queue.pop_front() {
        if arr[i] == 0 {
            return true;
        }
        visited[i] = true;
        if let Some(left) = i.checked_sub(arr[i] as usize) {
            if !visited[left] {
                queue.push_back(left);
            }
        }
        if arr.len() > i + arr[i] as usize && !visited[i + arr[i] as usize] {
            queue.push_back(i + arr[i] as usize);
        }
    }
    false
}

// initial DFS implementation
// fn helper(index: usize, arr: &Vec<i32>, visited: &mut Vec<bool>) -> bool {
//     if visited[index] {
//         return false;
//     }
//     if arr[index] == 0 {
//         return true;
//     }
//     visited[index] = true;
//     // yoda condition since < after usize is parsed as the start of a generic argument
//     if arr.len() > index + arr[index] as usize && helper(index + arr[index] as usize, arr, visited)
//     {
//         return true;
//     }
//     if index >= arr[index] as usize && helper(index - arr[index] as usize, arr, visited) {
//         return true;
//     }
//     false
// }

// pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
//     let mut visited = vec![false; arr.len()];
//     helper(start as usize, &arr, &mut visited)
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![4, 2, 3, 0, 3, 1, 2], 5, true),
            (vec![4, 2, 3, 0, 3, 1, 2], 0, true),
            (vec![3, 0, 2, 1, 2], 2, false),
        ];
        for (arr, start, expected) in test_tuples {
            assert_eq!(can_reach(arr, start), expected);
        }
    }
}
