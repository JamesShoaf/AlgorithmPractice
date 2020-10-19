/* 
You are the manager of a basketball team. For the upcoming tournament, you want to choose the team
with the highest overall score. The score of the team is the sum of scores of all the players in
the team.

However, the basketball team is not allowed to have conflicts. A conflict exists if a younger
player has a strictly higher score than an older player. A conflict does not occur between players
of the same age.

Given two lists, scores and ages, where each scores[i] and ages[i] represents the score and age of
the ith player, respectively, return the highest overall score of all possible basketball teams.
*/

use std::cmp;

pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    // sort players by age, then by score
    let mut ages_scores: Vec<(i32, i32)> = ages.into_iter()
        .zip(scores.into_iter())
        .collect();
    ages_scores.sort();
    // dp[i] represents the best score that can be made including player i.
    // it is score[i] + the maximum value of all teams containing older and better scoring
    // players which do not cause conflicts.
    let mut dp = vec![0; ages_scores.len()];
    for i in (0..dp.len()).rev() {
        dp[i] = ages_scores[i].1 + dp[i + 1..].iter().enumerate().filter(|&(j, _)| {
            ages_scores[j + i + 1].1 >= ages_scores[i].1 || ages_scores[j + i + 1].0 == ages_scores[i].0
        }).fold(0, |acc, (_, &max)| cmp::max(acc, max));
    }
    // the best team is then the max value in this vector.
    dp.into_iter().fold(0, |acc, max| cmp::max(acc, max))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            // (Vec::new(),
            // Vec::new(),
            // 0),
            (vec![1, 3, 5, 10, 15],
            vec![1, 2, 3, 4, 5],
            34),
            (vec![4, 5, 6, 5],
            vec![2, 1, 2, 1],
            16),
            (vec![1, 2, 3, 5],
            vec![8, 9, 10, 1],
            6),
        ];
        for (scores, ages, expected) in test_tuples {
            assert_eq!(best_team_score(scores, ages), expected);
        }
    }
}
