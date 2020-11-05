/* 
We have n chips, where the position of the ith chip is position[i].

We need to move all the chips to the same position. In one step, we can change the position of the
ith chip from position[i] to:

    position[i] + 2 or position[i] - 2 with cost = 0.
    position[i] + 1 or position[i] - 1 with cost = 1.

Return the minimum cost needed to move all the chips to the same position.
*/

use std::cmp;

pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
    let chips = position.len();
    // count chips at even position
    let even_position = position.into_iter()
        .filter(|pos| pos % 2 == 0)
        .count();
    // all even chips need to be moved to odd spaces, or all odd chips need to be moved to even
    // return min of odd and even count
    cmp::min(chips - even_position, even_position) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![1, 2, 3], 1),
            (vec![2, 2, 2, 3, 3], 2),
            (vec![1, 1000000000], 1),
            (vec![1, -1], 0),
            (Vec::new(), 0),
        ];
        for (position, expected) in test_tuples {
            assert_eq!(min_cost_to_move_chips(position), expected);
        }
    }
}
