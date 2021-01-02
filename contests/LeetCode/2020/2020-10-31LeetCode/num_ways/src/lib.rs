/* 
You are given a list of strings of the same length words and a string target.

Your task is to form target using the given words under the following rules:

    target should be formed from left to right.
    To form the ith character (0-indexed) of target, you can choose the kth character of the jth
    string in words if target[i] = words[j][k].
    Once you use the kth character of the jth string of words, you can no longer use the xth
    character of any string in words where x <= k. In other words, all characters to the left of or
    at index k become unusuable for every string.
    Repeat the process until you form the string target.

Notice that you can use multiple characters from the same string in words provided the conditions above are met.

Return the number of ways to form target from words. Since the answer may be too large, return it modulo 109 + 7.
*/

use std::collections::HashMap;

// initial memoized recursive solution (O(mn) time O(mn) space)

// fn helper(i: usize, j: usize,
//     words: &Vec<Vec<char>>, target: &Vec<char>,
//     counts: &Vec<HashMap<char,usize>>,
//     memo: &mut HashMap<(usize, usize), i32>
// ) -> i32 {
//     let modulus = 10_i32.pow(9) + 7;
//     if j == target.len() { return 1; }
//     if i >= words[0].len() { return 0; }
//     if let Some(&prev) = memo.get(&(i, j)) {
//         return prev;
//     }
//     let &matches = counts[i].get(&target[j]).unwrap_or(&0);
//     let mut count = helper(i + 1, j, words, target, counts, memo) % modulus;
//     for _ in 0..matches {
//         count += helper(i + 1, j + 1, words, target, counts, memo) % modulus;
//         count %= modulus;
//     }
//     memo.insert((i, j), count);
//     count
// }

// pub fn num_ways(words: Vec<String>, target: String) -> i32 {
//     let words: Vec<Vec<char>> = words.into_iter()
//         .map(|string| string.chars().collect())
//         .collect();
//     let target: Vec<char> = target.chars().collect();
//     let word_len = words[0].len();
//     let word_count = words.len();
//     let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
//     let counts: Vec<HashMap<char, usize>> = (0..word_len).map(|i| {
//         let mut counts: HashMap<char, usize> = HashMap::new();
//         for j in 0..word_count {
//             *counts.entry(words[j][i]).or_insert(0) += 1;
//         }
//         counts
//     }).collect();
//     helper(0, 0, &words, &target, &counts, &mut memo)
// }


// optimized solution (O(mn) time O(max(m, n)) space)
pub fn num_ways(words: Vec<String>, target: String) -> i32 {
    if words.len() == 0 { return 0; }
    let modulus: i64 = 10_i64.pow(9) + 7;
    let words: Vec<Vec<char>> = words.into_iter()
        .map(|string| string.chars().collect())
        .collect();
    let target: Vec<char> = target.chars().collect();
    // dp[i] is the number of ways to make a string matching the first i characters of target
    let mut dp = vec![0; target.len() + 1];
    dp[0] = 1;
    for i in 0..words[0].len() {
        let mut count: HashMap<char, i64> = HashMap::new();
        for j in 0..words.len() {
            *count.entry(words[j][i]).or_insert(0) += 1;
        }
        for j in (0..target.len()).rev() {
            dp[j + 1] += dp[j] * count.get(&target[j]).unwrap_or(&0);
            dp[j + 1] %= modulus;
        }
    }
    dp[target.len()] as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
