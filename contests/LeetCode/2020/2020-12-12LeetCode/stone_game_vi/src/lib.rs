/*
Alice and Bob take turns playing a game, with Alice starting first.

There are n stones in a pile. On each player's turn, they can remove a stone from the pile and
receive points based on the stone's value. Alice and Bob may value the stones differently.

You are given two integer arrays of length n, aliceValues and bobValues. Each aliceValues[i] and
bobValues[i] represents how Alice and Bob, respectively, value the ith stone.

The winner is the person with the most points after all the stones are chosen. If both players have
the same amount of points, the game results in a draw. Both players will play optimally.

Determine the result of the game, and:

    If Alice wins, return 1.
    If Bob wins, return -1.
    If the game results in a draw, return 0.

*/

use std::cmp::Ordering;
use std::collections::BinaryHeap;

// the best stone for either player to take is the stone with the greatest value to both combined
// if alice takes a stone that bob values at 5, those are 5 points alice doesn't have to gather
// from other stones in order to win.
pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
    let mut queue: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
    for (a, b) in alice_values.into_iter().zip(bob_values.into_iter()) {
        queue.push((a + b, a, b));
    }
    let mut alice_score = 0;
    let mut bob_score = 0;
    let mut alice_turn = true;
    while let Some((_, a, b)) = queue.pop() {
        if alice_turn {
            alice_score += a;
        } else {
            bob_score += b;
        }
        alice_turn = !alice_turn;
    }
    match alice_score.cmp(&bob_score) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
