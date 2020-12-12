/*
You are given a string allowed consisting of distinct characters and an array of strings words. A
string is consistent if all characters in the string appear in the string allowed.

Return the number of consistent strings in the array words.
*/

use std::collections::HashSet;

pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut allowed_chars = HashSet::new();
    for c in allowed.chars() {
        allowed_chars.insert(c);
    }
    words
        .into_iter()
        .filter(|word| word.chars().all(|c| allowed_chars.contains(&c)))
        .collect::<Vec<String>>()
        .len() as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
