use std::collections::HashSet;
use std::iter::FromIterator;

fn helper(i: usize, chars: &Vec<char>, dict: &HashSet<String>, memo: &mut Vec<bool>) -> bool {
    if i == chars.len() {
        return true;
    }
    if memo[i] == false {
        return false;
    }
    let mut curr = String::new();
    for j in i..chars.len() {
        curr.push(chars[j]);
        if dict.contains(&curr) && helper(j + 1, chars, dict, memo) {
            return true;
        }
    }
    memo[i] = false;
    false
}

fn all_letters_contained(s: &String, words: &Vec<String>) -> bool {
    let mut char_set: HashSet<char> = HashSet::new();
    for word in words.iter() {
        for c in word.chars() {
            char_set.insert(c);
        }
    }
    s.chars().all(|c| char_set.contains(&c))
}

pub fn word_break(s: String, words: Vec<String>) -> bool {
    if !all_letters_contained(&s, &words) {
        return false; 
    }
    let mut memo: Vec<bool> = vec![true; s.len()];
    let dict: HashSet<String> = HashSet::from_iter(words.into_iter());
    let chars = s.chars().collect();
    helper(0, &chars, &dict, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                String::from("leetcode"),
                vec![String::from("leet"), String::from("code")],
                true,
            ),
            (
                String::from("applepenapple"),
                vec![String::from("apple"), String::from("pen")],
                true,
            ),
            (
                String::from("catsandog"),
                vec![
                    String::from("cats"),
                    String::from("dog"),
                    String::from("sand"),
                    String::from("and"),
                    String::from("cat"),
                ],
                false,
            ),
            (
                String::from("aaaaaaaaaaaaaaaab"),
                vec![
                    String::from("a"),
                    String::from("aa"),
                    String::from("aaa"),
                    String::from("aaaa"),
                    String::from("aaaaa"),
                ],
                false,
            ),
        ];
        for (s, words, expected) in test_tuples {
            assert_eq!(word_break(s, words), expected);
        }
    }
}

// #1 solution for comparison

// pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
//     let mut string: Vec<char> = s.chars().collect();
//     let mut steps: Vec<bool> = vec![false; string.len()];
//     let word_dict: Vec<Vec<char>> = word_dict.iter()
//         .map(|word| word.chars().collect())
//         .collect();
    
//     for i in 0..string.len() {
//         if i > 0 && !steps[i-1] {
//             continue;
//         }
        
//         for word in &word_dict {
//             if i + word.len() > string.len() {
//                 continue;
//             }
            
//             let mut valid = true;
            
//             for (j, c) in word.iter().enumerate() {
//                 if string[i+j] != *c {
//                     valid = false;
//                     break;
//                 }
//             }
            
//             if valid {
//                 steps[i + word.len() - 1] = true;
//             }
//         }
//     }
    
//     return steps[steps.len() - 1];
// }