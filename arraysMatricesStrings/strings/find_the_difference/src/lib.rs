/* 
 Given two strings s and t which consist of only lowercase letters.

String t is generated by random shuffling string s and then add one more letter at a random position.

Find the letter that was added in t.
*/

use std::collections::HashMap;


// return either the first character from t not in s, or '-'
pub fn find_the_difference(s: String, t: String) -> char {
    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    for c in t.chars() {
        if let Some(count) = counts.get_mut(&c) {
            *count -= 1;
            if *count >= 0 { continue; }
        }
        return c;
    }
    '-'
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("abcd"), String::from("abcde"), 'e'),
            (String::from("a"), String::from("aba"), 'b'),
            (String::from("a"), String::from("aa"), 'a'),
            (String::from("aa"), String::from("aa"), '-'),
        ];
        for (s, t, expected) in test_tuples {
            assert_eq!(find_the_difference(s, t), expected);
        }
    }
}
