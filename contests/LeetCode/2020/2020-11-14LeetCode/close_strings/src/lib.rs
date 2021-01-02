/*
Two strings are considered close if you can attain one from the other using the following operations:

    Operation 1: Swap any two existing characters.
        For example, abcde -> aecdb
    Operation 2: Transform every occurrence of one existing character into another existing character, and do the same with the other character.
        For example, aacabb -> bbcbaa (all a's turn into b's, and all b's turn into a's)

You can use the operations on either string as many times as necessary.

Given two strings, word1 and word2, return true if word1 and word2 are close, and false otherwise.
*/

use std::collections::HashMap;

pub fn close_strings(word1: String, word2: String) -> bool {
    let mut counts1: HashMap<char, usize> = HashMap::new();
    for c in word1.chars() {
        *counts1.entry(c).or_insert(0) += 1;
    }
    let mut counts2: HashMap<char, usize> = HashMap::new();
    for c in word2.chars() {
        *counts2.entry(c).or_insert(0) += 1;
    }
    let mut keys1: Vec<&char> = counts1.keys().collect();
    keys1.sort();
    let mut keys2: Vec<&char> = counts2.keys().collect();
    keys2.sort();
    if keys1 != keys2 {
        return false;
    }
    let mut values1: Vec<&usize> = counts1.values().collect();
    values1.sort();
    let mut values2: Vec<&usize> = counts2.values().collect();
    values2.sort();
    if values1 != values2 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
