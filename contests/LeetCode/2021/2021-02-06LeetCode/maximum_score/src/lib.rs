/*
You are playing a solitaire game with three piles of stones of sizes
a​​​​​​, b,​​​​​​ and c​​​​​​ respectively. Each turn you choose two different non-empty piles,
take one stone from each, and add 1 point to your score. The game stops when
there are fewer than two non-empty piles (meaning there are no more available moves).

Given three integers a​​​​​, b,​​​​​ and c​​​​​, return the maximum score you can get.
*/

use std::collections::BinaryHeap;

pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
    let mut score = 0;
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    heap.push(a);
    heap.push(b);
    heap.push(c);
    while heap.len() > 1 {
        score += 1;
        let first = heap.pop().unwrap() - 1;
        let second = heap.pop().unwrap() - 1;
        if first > 0 {
            heap.push(first);
        }
        if second > 0 {
            heap.push(second);
        }
    }
    score
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
