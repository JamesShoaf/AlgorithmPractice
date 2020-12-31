/*
Given n non-negative integers representing the histogram's bar height where the width of each bar
is 1, find the area of largest rectangle in the histogram.
*/

// stack based implementation (O(n) time, O(n) space)

use std::cmp;
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut max_area: usize = 0;
    for (idx, height) in heights.iter().enumerate() {
        let height = *height as usize;
        let mut start_idx = idx;
        // maintain a stack of ascending heights
        while let Some(&(prev_idx, prev_height)) = stack.last() {
            if height > prev_height {
                break;
            }
            // when an equal or lesser height is reached, pop off the top height
            stack.pop();
            // calculate the area from the start of that slice to the current index
            let area = prev_height * (idx - prev_idx);
            max_area = cmp::max(max_area, area);
            // set the start of the current slice to the start of the previous slice
            start_idx = prev_idx;
        }
        // push on the first index of the current height and the current height
        stack.push((start_idx, height));
    }

    // for any remaining slices, calculate the area from the start of that slice to the end
    while let Some((idx, height)) = stack.pop() {
        let area = height * (heights.len() - idx);
        max_area = cmp::max(max_area, area);
    }

    max_area as i32
}

// initial O(n^2) implementation

// pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
//     let mut best_from_index: Vec<usize> = Vec::new();
//     for i in 0..heights.len() {
//         let mut height = heights[i] as usize;
//         let mut best = height;
//         for j in i + 1..heights.len() {
//             if height == 0 {
//                 break;
//             }
//             height = cmp::min(height, heights[j] as usize);
//             best = cmp::max(best, (j - i + 1) * height);
//         }
//         best_from_index.push(best);
//     }
//     best_from_index.into_iter().max().unwrap_or(0) as i32
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![2, 1, 5, 6, 2, 3], 10),
            (vec![4, 1, 3, 1, 1, 5], 6),
            (vec![4, 1, 3, 1, 5], 5),
        ];
        for (heights, expected) in test_tuples {
            assert_eq!(super::largest_rectangle_area(heights), expected);
        }
    }
}
