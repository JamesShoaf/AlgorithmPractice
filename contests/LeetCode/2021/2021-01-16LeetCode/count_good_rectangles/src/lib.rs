/*
You are given an array rectangles where rectangles[i] = [li, wi] represents the ith rectangle of
length li and width wi.

You can cut the ith rectangle to form a square with a side length of k if both k <= li and k <= wi.
For example, if you have a rectangle [4,6], you can cut it to get a square with a side length of at
most 4.

Let maxLen be the side length of the largest square you can obtain from any of the given rectangles.

Return the number of rectangles that can make a square with a side length of maxLen.
*/

use std::cmp;

pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
    let squares: Vec<i32> = rectangles
        .into_iter()
        .map(|vec| cmp::min(vec[0], vec[1]))
        .collect();
    let max = *squares.iter().max().unwrap_or(&0);
    squares.into_iter().filter(|&v| v == max).count() as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
