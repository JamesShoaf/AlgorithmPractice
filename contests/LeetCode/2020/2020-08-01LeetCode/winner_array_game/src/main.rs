// Given an integer array arr of distinct integers and an integer k.

// A game will be played between the first two elements of the array (i.e. arr[0] and arr[1]).
// In each round of the game, we compare arr[0] with arr[1], the larger integer wins and remains
// at position 0 and the smaller integer moves to the end of the array. The game ends when an
// integer wins k consecutive rounds.

// Return the integer which will win the game.

// It is guaranteed that there will be a winner of the game.

struct Solution {}

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        if k as usize >= arr.len() - 1 { return *arr.iter().max().unwrap(); }
        let mut winner = arr[0];
        let mut win_count = 0;
        for i in 1..arr.len() {
            if win_count >= k { return winner; }
            if winner > arr[i] { win_count += 1; } 
            else {
                winner = arr[i];
                win_count = 1;
            }
        }
        winner
    }
}

fn main() {
    println!("Hello, world!");
}
