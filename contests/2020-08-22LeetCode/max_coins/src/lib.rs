/* 
There are 3n piles of coins of varying size, you and your friends will take piles of coins as follows:

    In each step, you will choose any 3 piles of coins (not necessarily consecutive).
    Of your choice, Alice will pick the pile with the maximum number of coins.
    You will pick the next pile with maximum number of coins.
    Your friend Bob will pick the last pile.
    Repeat until there are no more piles of coins.

Given an array of integers piles where piles[i] is the number of coins in the ith pile.

Return the maximum number of coins which you can have.
*/

fn max_coins(mut piles: Vec<i32>) -> i32 {
    piles.sort();
    let mut output = 0;
    let mut i = piles.len();
    for _ in 0..piles.len()/3 {
        i -= 2;
        output += piles[i];
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![2,4,1,2,7,8], 9),
            (vec![2,4,5], 4),
            (vec![9,8,7,6,5,1,2,3,4], 18),
        ];
        for (piles, expected) in test_tuples {
            assert_eq!(max_coins(piles), expected);
        }
    }
}
