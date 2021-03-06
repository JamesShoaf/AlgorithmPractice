/*
The beauty of a string is the difference in frequencies between the most frequent and least
frequent characters.

    For example, the beauty of "abaacc" is 3 - 1 = 2.

Given a string s, return the sum of beauty of all of its substrings.
*/

use std::collections::HashMap;

pub fn beauty_sum(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut res = 0;
    for i in 0..chars.len() {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for j in i..chars.len() {
            *counts.entry(chars[j]).or_insert(0) += 1;
            res += counts.values().max().unwrap() - counts.values().min().unwrap();
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
