/*
Alice and Bob take turns playing a game, with Alice starting first.

There are n stones arranged in a row. On each player's turn, they can remove either the leftmost
stone or the rightmost stone from the row and receive points equal to the sum of the remaining
stones' values in the row. The winner is the one with the higher score when there are no stones
left to remove.

Bob found that he will always lose this game (poor Bob, he always loses), so he decided to minimize
the score's difference. Alice's goal is to maximize the difference in the score.

Given an array of integers stones where stones[i] represents the value of the ith stone from the
left, return the difference in Alice and Bob's score if they both play optimally.
*/

use std::collections::HashMap;

fn recursive_helper(
    l: usize,
    r: usize,
    rem: i32,
    memo: &mut HashMap<(usize, usize), (i32, i32)>,
    stones: &Vec<i32>,
) -> (i32, i32) {
    if l == r {
        return (0, 0);
    }
    if let Some(prev) = memo.get(&(l, r)) {
        return *prev;
    }
    let rem_left = rem - stones[l];
    let rem_right = rem - stones[r];
    let (a_left, b_left) = recursive_helper(l + 1, r, rem_left, memo, stones);
    let (a_right, b_right) = recursive_helper(l, r - 1, rem_right, memo, stones);
    // alice's turn
    if (r - l) % 2 != stones.len() % 2 {
        let res = if a_left + rem_left - b_left > a_right + rem_right - b_right {
            (a_left + rem_left, b_left)
        } else {
            (a_right + rem_right, b_right)
        };
        memo.insert((l, r), res);
        return res;
    }
    let res = if a_left - b_left - rem_left < a_right - b_right - rem_right {
        (a_left, b_left + rem_left)
    } else {
        (a_right, b_right + rem_right)
    };
    memo.insert((l, r), res);
    return res;
}

pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
    let sum = stones.iter().sum();
    let mut memo = HashMap::new();
    let res = recursive_helper(0, stones.len() - 1, sum, &mut memo, &stones);
    res.0 - res.1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
