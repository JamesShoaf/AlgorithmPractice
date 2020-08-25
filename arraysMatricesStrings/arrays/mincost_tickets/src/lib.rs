/* 
In a country popular for train travel, you have planned some train travelling one year in advance. 
The days of the year that you will travel is given as an array days.  Each day is an integer from 1 to 365.

Train tickets are sold in 3 different ways:

    a 1-day pass is sold for costs[0] dollars;
    a 7-day pass is sold for costs[1] dollars;
    a 30-day pass is sold for costs[2] dollars.

The passes allow that many days of consecutive travel.  For example, if we get a 7-day pass on day 2,
then we can travel for 7 days: day 2, 3, 4, 5, 6, 7, and 8.

Return the minimum number of dollars you need to travel every day in the given list of days.
*/

use std::collections::HashMap;
use std::cmp;

fn dfs(next_uncovered: i32, last_i: usize, days: &Vec<i32>, costs: &Vec<(i32, i32)>, memo: &mut HashMap<i32, i32>) -> i32 {
    if let Some(prev) = memo.get(&next_uncovered) {
        return *prev;
    }
    let mut first_uncovered_travel = 0;
    let mut next_i = 0;
    for i in last_i..days.len() {
        if days[i] >= next_uncovered {
            first_uncovered_travel = days[i];
            next_i = i;
            break;
        }
    }
    if first_uncovered_travel == 0 {
        memo.insert(next_uncovered, 0);
        return 0;
    }
    let mut mincost = i32::MAX;
    for (dur, cost) in costs.iter() {
        mincost = cmp::min(mincost, cost + dfs(first_uncovered_travel + dur, next_i, days, costs, memo));
    }
    memo.insert(next_uncovered, mincost);
    mincost
}

fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();
    let duration_costs: Vec<(i32, i32)> = vec![
        (1, costs[0]),
        (7, costs[1]),
        (30, costs[2]),
    ];
    dfs(1, 0, &days, &duration_costs, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), vec![2, 7, 15], 0),
            (vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15], 11),
            (vec![1, 4, 6, 7, 8, 20], vec![2, 7, 1], 1),
            (vec![1, 4, 6, 7, 8, 20], vec![2, 1, 15], 3),
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15], 17),
        ];
        for (days, costs, expected) in test_tuples {
            assert_eq!(super::mincost_tickets(days, costs), expected);
        }
    }
}
