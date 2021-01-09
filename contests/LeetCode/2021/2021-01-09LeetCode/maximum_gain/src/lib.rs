/*
You are given a string s and two integers x and y. You can perform two types of operations any number of times.

    Remove substring "ab" and gain x points.
        For example, when removing "ab" from "cabxbae" it becomes "cxbae".
    Remove substring "ba" and gain y points.
        For example, when removing "ba" from "cabxbae" it becomes "cabxe".

Return the maximum points you can gain after applying the above operations on s.
*/

fn helper(s: Vec<char>, val: i32, c_1: char, c_2: char, val_2: i32, r2_flag: bool) -> i32 {
    let mut res = 0;
    let mut stack = Vec::new();
    for c in s.into_iter() {
        if c == c_2 && !stack.is_empty() && *stack.last().unwrap() == c_1 {
            stack.pop();
            res += val;
        } else {
            stack.push(c);
        }
    }
    res + if !r2_flag {
        helper(stack, val_2, c_2, c_1, val, true)
    } else {
        0
    }
}

pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    let s = s.chars().collect();
    if x >= y {
        helper(s, x, 'a', 'b', y, false)
    } else {
        helper(s, y, 'b', 'a', x, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            ("cdbcbbaabab".to_string(), 4, 5, 19),
            ("aabbaaxybbaabb".to_string(), 5, 4, 20),
        ];
        for (s, x, y, expected) in test_tuples {
            assert_eq!(maximum_gain(s, x, y), expected);
        }
    }
}

// initial O(n^2) implementation - Generates a new subarray each time a pair is removed

// use std::cmp;
// use std::collections::HashMap;

// fn recursive(arr: Vec<char>, x: i32, y: i32, map: &mut HashMap<Vec<char>, i32>) -> i32 {
//     if arr.len() < 2 {
//         return 0;
//     }
//     if let Some(&prev) = map.get(&arr) {
//         return prev;
//     }
//     let mut res = 0;
//     for i in 1..arr.len() {
//         if arr[i - 1] == 'a' && arr[i] == 'b' {
//             let mut p1: Vec<char> = arr[..i - 1].iter().map(|&c| c).collect();
//             if i + 1 < arr.len() {
//                 let p2: Vec<char> = arr[i + 1..].iter().map(|&c| c).collect();
//                 p1.extend(p2);
//             }
//             res = cmp::max(res, x + recursive(p1, x, y, map));
//         }
//         if arr[i - 1] == 'b' && arr[i] == 'a' {
//             let mut p1: Vec<char> = arr[..i - 1].iter().map(|&c| c).collect();
//             if i + 1 < arr.len() {
//                 let p2: Vec<char> = arr[i + 1..].iter().map(|&c| c).collect();
//                 p1.extend(p2);
//             }
//             res = cmp::max(res, y + recursive(p1, x, y, map));
//         }
//     }
//     map.insert(arr, res);
//     res
// }

// pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
//     let mut res = 0;
//     let mut map: HashMap<Vec<char>, i32> = HashMap::new();
//     for substring in s.split(|c| c != 'a' && c != 'b') {
//         let arr: Vec<char> = substring.chars().collect();
//         res += recursive(arr, x, y, &mut map);
//     }
//     res
// }
