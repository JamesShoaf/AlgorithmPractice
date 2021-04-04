/*
You are given the logs for users' actions on LeetCode, and an integer k. The logs are represented by a 2D integer array logs where each logs[i] = [IDi, timei] indicates that the user with IDi performed an action at the minute timei.

Multiple users can perform actions simultaneously, and a single user can perform multiple actions in the same minute.

The user active minutes (UAM) for a given user is defined as the number of unique minutes in which the user performed an action on LeetCode. A minute can only be counted once, even if multiple actions occur during it.

You are to calculate a 1-indexed array answer of size k such that, for each j (1 <= j <= k), answer[j] is the number of users whose UAM equals j.

Return the array answer as described above.
*/

use std::collections::{HashMap, HashSet};

pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut res = vec![0; k];
    for log in logs {
        let (user, time) = (log[0], log[1]);
        map.entry(user).or_insert(HashSet::new()).insert(time);
    }
    for (_, set) in map {
        if set.len() <= k {
            res[set.len() - 1] += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
