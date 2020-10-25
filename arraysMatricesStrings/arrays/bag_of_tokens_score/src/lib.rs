/* 
You have an initial power of P, an initial score of 0, and a bag of tokens where tokens[i] is the value of the ith token (0-indexed).

Your goal is to maximize your total score by potentially playing each token in one of two ways:

    If your current power is at least tokens[i], you may play the ith token face up, losing tokens[i] power and gaining 1 score.
    If your current score is at least 1, you may play the ith token face down, gaining tokens[i] power and losing 1 score.

Each token may be played at most once and in any order. You do not have to play all the tokens.

Return the largest possible score you can achieve after playing any number of tokens.
*/
use std::collections::VecDeque;

pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut p: i32) -> i32 {
    // sort the tokens and convert them to a deque
    tokens.sort();
    let mut tokens: VecDeque<i32> = tokens.into_iter().collect();
    let mut best_score = 0;
    let mut current_score = 0;
    while !tokens.is_empty() {
        match p >= tokens[0] {
            // convert the lowest cost token to points if possible
            true => { 
                p -= tokens.pop_front().unwrap();
                current_score += 1;
                // update the best_score
                if current_score > best_score {
                    best_score = current_score;
                }
            }
            // return the best score reached if score would go negative
            false if current_score == 0 => return best_score,
            // otherwise, convert the highest cost token to power
            false => {
                p += tokens.pop_back().unwrap();
                current_score -= 1;
            }
        }
    }
    best_score
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), 1000, 0),
            (vec![100], 50, 0),
            (vec![200, 100], 150, 1),
            (vec![71, 55, 82], 54, 0),
            (vec![300, 200, 100, 400], 200, 2),
            (vec![300, 200, 100, 400], 1000, 4),
        ];
        for (tokens, p, expected) in test_tuples {
            assert_eq!(bag_of_tokens_score(tokens, p), expected);
        }
    }
}
