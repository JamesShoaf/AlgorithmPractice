/* 
Alice and Bob take turns playing a game, with Alice starting first.

Initially, there are n stones in a pile.  On each player's turn, that player makes a move
consisting of removing any non-zero square number of stones in the pile.

Also, if a player cannot make a move, he/she loses the game.

Given a positive integer n. Return True if and only if Alice wins the game otherwise return False,
assuming both players play optimally.
*/

pub fn stone_game_iv(n: i32) -> bool {
    assert!(n >= 0, "n cannot be less than 0!");
    let mut memo = vec![false];
    for i in 1..=n {
        memo.push(
            (1..=i)
                .map(|j| j.pow(2) )
                .take_while(|&j| j <= i)
                .any(|j| !memo[(i - j) as usize])
        );
    }
    memo[n as usize]
}

// initial implementation
// use std::collections::HashMap;

// fn recursive(n: i32, memo: &mut HashMap<i32, bool>) -> bool {
//     if let Some(&prev) = memo.get(&n) {
//         return prev;
//     }
//     let res = (1..=n)
//         .map(|i| i.pow(2) )
//         .take_while(|&i| i <= n)
//         .any(|i| !recursive(n - i, memo));
//     memo.insert(n, res);
//     res
// }

// pub fn stone_game_iv(n: i32) -> bool {
//     let mut memo: HashMap<i32, bool> = HashMap::new();
//     memo.insert(0, false);
//     recursive(n, &mut memo)
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (0, false),
            (1, true),
            (2, false),
            (4, true),
            (7, false),
            (17, false),
            (1000, true),
            (10000, true),
            (100000, true),
        ];
        for (n, expected) in test_tuples {
            assert_eq!(stone_game_iv(n), expected);
        }
    }
}
