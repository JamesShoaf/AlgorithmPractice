/*
On a social network consisting of m users and some friendships between users, two users can
communicate with each other if they know a common language.

You are given an integer n, an array languages, and an array friendships where:

    There are n languages numbered 1 through n,
    languages[i] is the set of languages the i​​​​​​th​​​​ user knows, and
    friendships[i] = [u​​​​​​i​​​, v​​​​​​i] denotes a friendship between the users u​​​​​​​​​​​i​​​​​ and vi.

You can choose one language and teach it to some users so that all friends can communicate with
each other. Return the minimum number of users you need to teach.
Note that friendships are not transitive, meaning if x is a friend of y and y is a friend of z,
this doesn't guarantee that x is a friend of z.
*/

/*
Constraints:

    2 <= n <= 500
    languages.length == m
    1 <= m <= 500
    1 <= languages[i].length <= n
    1 <= languages[i][j] <= n
    1 <= u​​​​​​i < v​​​​​​i <= languages.length
    1 <= friendships.length <= 500
    All tuples (u​​​​​i, v​​​​​​i) are unique
    languages[i] contains only unique values

*/

use std::cmp;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
    const M: usize = 501;
    let mut res = 501;
    // transform languages from vec of vecs to vec of HashSets (for faster lookup)
    let languages: Vec<HashSet<i32>> = languages
        .into_iter()
        .map(|vec| HashSet::from_iter(vec.into_iter()))
        .collect();
    // filter out any friendships which already share a language
    let friendships: Vec<Vec<i32>> = friendships
        .into_iter()
        .filter(|rel| languages[rel[0] as usize - 1].is_disjoint(&languages[rel[1] as usize - 1]))
        .collect();
    // for each language
    for lang in 1..n + 1 {
        // create an array of whether each friend needs to be taught the language
        let mut to_teach = [false; M];
        // for each friendship, check whether one, both, or neither friend needs
        // to learn, then update the array
        for rel in &friendships {
            let friend_1 = rel[0] as usize;
            let friend_2 = rel[1] as usize;
            if !to_teach[friend_1] && !languages[friend_1 - 1].contains(&lang) {
                to_teach[friend_1] = true;
            }
            if !to_teach[friend_2] && !languages[friend_2 - 1].contains(&lang) {
                to_teach[friend_2] = true;
            }
        }
        // sum the number which need to be taught
        res = cmp::min(res, to_teach.iter().filter(|&&k| k).count() as i32);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                2,
                vec![vec![1], vec![2], vec![1, 2]],
                vec![vec![1, 2], vec![1, 3], vec![2, 3]],
                1,
            ),
            (
                3,
                vec![vec![2], vec![1, 3], vec![1, 2], vec![3]],
                vec![vec![1, 4], vec![1, 2], vec![3, 4], vec![2, 3]],
                2,
            ),
        ];
        for (n, languages, friendships, expected) in test_tuples {
            assert_eq!(minimum_teachings(n, languages, friendships), expected);
        }
    }
}
