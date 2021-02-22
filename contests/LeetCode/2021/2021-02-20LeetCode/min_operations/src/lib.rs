/*
You have n boxes. You are given a binary string boxes of length n, where boxes[i] is '0'
if the ith box is empty, and '1' if it contains one ball.

In one operation, you can move one ball from a box to an adjacent box. Box i is adjacent
to box j if abs(i - j) == 1. Note that after doing so, there may be more than one ball in
some boxes.

Return an array answer of size n, where answer[i] is the minimum number of operations
needed to move all the balls to the ith box.

Each answer[i] is calculated considering the initial state of the boxes.
*/

pub fn min_operations(boxes: String) -> Vec<i32> {
    let boxes: Vec<bool> = boxes.chars().map(|c| c.to_digit(2).unwrap() == 1).collect();
    let mut left_moves = vec![0; boxes.len()];
    let mut count = 0;
    for i in 0..boxes.len() {
        if i > 0 {
            left_moves[i] += left_moves[i - 1];
        }
        left_moves[i] += count;
        if boxes[i] {
            count += 1;
        }
    }
    count = 0;
    let mut right_moves = vec![0; boxes.len()];
    for i in (0..boxes.len()).rev() {
        if i + 1 < boxes.len() {
            right_moves[i] += right_moves[i + 1];
        }
        right_moves[i] += count;
        if boxes[i] {
            count += 1;
        }
    }
    for i in 0..boxes.len() {
        left_moves[i] += right_moves[i];
    }
    left_moves
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
