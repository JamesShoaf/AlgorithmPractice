/* 
Given a string s, return the maximum number of unique substrings that the given string can be split into.

You can split string s into any list of non-empty substrings, where the concatenation of the substrings forms the original string. However, you must split the substrings such that all of them are unique.

A substring is a contiguous sequence of characters within a string.
*/

use std::collections::HashSet;
use std::cmp;

fn helper(index: usize, chars: &Vec<char>, set: &mut HashSet<String>, stack: &mut Vec<String>, max: &mut usize) {
    if index == chars.len() {
        *max = cmp::max(*max, stack.len());
        return;
    }
    let mut next_split = String::from("");
    for i in index..chars.len() {
        next_split.push(chars[i]);
        if !set.contains(&next_split) {
            set.insert(next_split.clone());
            stack.push(next_split.clone());
            helper(i + 1, chars, set, stack, max);
            stack.pop();
            set.remove(&next_split);
        }
    }
}

pub fn max_unique_split(s: String) -> i32 {
    let mut stack: Vec<String> = Vec::new();
    let mut set: HashSet<String> = HashSet::new();
    let chars: Vec<char> = s.chars().collect();
    let mut max: usize = 1;
    helper(0, &chars, &mut set, &mut stack, &mut max);
    max as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
