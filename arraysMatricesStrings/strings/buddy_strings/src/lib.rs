/* 
Given two strings A and B of lowercase letters, return true if you can swap two letters in A so the result is equal to B, otherwise, return false.
*/

use std::collections::HashMap;

pub fn buddy_strings(a: String, b:String) -> bool {
    let mut counts: HashMap<char, i32> = HashMap::new();
    // count characters in a
    for c in a.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    // if the strings are the same, and there are any duplicates, return true (aa can be swapped to aa)
    if a == b && counts.iter().any(|(_, &count)| count > 1) { return true; }
    // check if the strings contain the same characters
    for c in b.chars() {
        *counts.entry(c).or_insert(0) -= 1;
    }
    if counts.into_iter().any(|(_, count)| count != 0) { return false; }
    // count errors
    a.chars().zip(b.chars()).fold(0, |acc, (a, b)| if a != b { acc + 1 } else { acc }) == 2
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("ab"), String::from("ba"), true),
            (String::from("ab"), String::from("ab"), false),
            (String::from("aa"), String::from("aa"), true),
            (String::from("aaaaaaabc"), String::from("aaaaaaacb"), true),
            (String::new(), String::from("aa"), false),
        ];
        for (a, b, expected) in test_tuples {
            assert_eq!(buddy_strings(a, b), expected);
        }
    }
}
