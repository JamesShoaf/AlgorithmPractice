/*
Given two strings s and t, your goal is to convert s into t in k moves or less.

During the ith (1 <= i <= k) move you can:

    Choose any index j (1-indexed) from s, such that 1 <= j <= s.length and j has not been chosen
    in any previous move, and shift the character at that index i times.
    Do nothing.

Shifting a character means replacing it by the next letter in the alphabet (wrapping around so that
    'z' becomes 'a'). Shifting a character by i means applying the shift operations i times.

Remember that any index j can be picked at most once.

Return true if it's possible to convert s into t in no more than k moves, otherwise return false.
*/

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        if s.len() != t.len() { return false; }
        let abecedary = String::from("abcdefghijklmnopqrstuvwxyz");
        let mut abecedary = abecedary.chars();
        let mut char_map: HashMap<char, i32> = HashMap::new();
        for i in 0..26 {
            char_map.insert(abecedary.next().unwrap(), i);
        }
        let s_chars = s.chars();
        let t_chars = t.chars();
        let mut shift_map: HashMap<i32, i32> = HashMap::new();

        for (s_char, t_char) in s_chars.zip(t_chars) {
            let s_val = char_map.get(&s_char).unwrap();
            let t_val = char_map.get(&t_char).unwrap();
            let mut shift = t_val - s_val;
            if shift < 0 { shift += 26; }
            if shift > 0 {
                let updated = shift_map.entry(shift).or_insert(shift - 26);
                *updated += 26;
                if *updated > k { return false; }
            }
        }
        true
    }
}

fn main() {
    let s = String::from("input");
    let t = String::from("ouput");
    let k = 9;
    println!("{}", Solution::can_convert_string(s, t, k));
}
