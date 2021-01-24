/*
You are given two strings a and b that consist of lowercase letters. In one operation,
you can change any character in a or b to any lowercase letter.

Your goal is to satisfy one of the following three conditions:

    Every letter in a is strictly less than every letter in b in the alphabet.
    Every letter in b is strictly less than every letter in a in the alphabet.
    Both a and b consist of only one distinct letter.

Return the minimum number of operations needed to achieve your goal.
*/

use std::cmp;
use std::collections::HashMap;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const ALPHABET_LEN: usize = 26;

fn get_counts(s: &String) -> Vec<(char, u32)> {
    let mut counts: HashMap<char, u32> = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut counts: Vec<(char, u32)> = counts.into_iter().collect();
    counts.sort_unstable();
    counts
}

fn single_char_cost(counts: &Vec<(char, u32)>) -> u32 {
    let len = counts.iter().map(|&(_, count)| count).sum::<u32>();
    let max = *counts.iter().map(|(_, count)| count).max().unwrap_or(&0);
    len - max
}

fn cost_to_lower(counts: &Vec<(char, u32)>) -> [u32; ALPHABET_LEN] {
    let abecedary: Vec<char> = ALPHABET.chars().collect();
    let mut res = [0; ALPHABET_LEN];
    let mut counts_ptr = counts.len() as isize - 1;
    for i in (0..ALPHABET_LEN - 1).rev() {
        res[i] += res[i + 1];
        if counts_ptr >= 0 && abecedary[i] < counts[counts_ptr as usize].0 {
            res[i] += counts[counts_ptr as usize].1;
            counts_ptr -= 1;
        }
    }
    res
}

fn cost_to_raise(counts: &Vec<(char, u32)>) -> [u32; ALPHABET_LEN] {
    let abecedary: Vec<char> = ALPHABET.chars().collect();
    let mut res = [0; ALPHABET_LEN];
    let mut counts_ptr = 0;
    for i in 1..ALPHABET_LEN {
        res[i] += res[i - 1];
        if counts_ptr < counts.len() && abecedary[i] > counts[counts_ptr].0 {
            res[i] += counts[counts_ptr].1;
            counts_ptr += 1;
        }
    }
    res
}

fn cost_to_make_first_greater(a: &[u32; ALPHABET_LEN], b: &[u32; ALPHABET_LEN], res: &mut u32) {
    for i in 0..ALPHABET_LEN - 1 {
        *res = cmp::min(*res, a[i + 1] + b[i]);
    }
}

pub fn min_characters(a: String, b: String) -> i32 {
    let a_counts = get_counts(&a);
    let b_counts = get_counts(&b);
    let mut res = single_char_cost(&a_counts) + single_char_cost(&b_counts);
    cost_to_make_first_greater(
        &cost_to_raise(&a_counts),
        &cost_to_lower(&b_counts),
        &mut res,
    );
    cost_to_make_first_greater(
        &cost_to_raise(&b_counts),
        &cost_to_lower(&a_counts),
        &mut res,
    );
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_characters() {
        let test_tuples = vec![
            ("aba".to_string(), "caa".to_string(), 2),
            ("dabadd".to_string(), "cda".to_string(), 3),
            (
                "jukdyrwxmayusovrggihfiluaewjbixpxybjfsjuyjcdnsxacodbwfdbfyklwfkblnijmhwivo".to_string(),
                "sdtinjseqrjmmumheuimgmnwfjgwftdldjwpugupnwnltslplgufmynmsovqnculunfycwlxrcregkwkvlwwkhitqyiavabxhu".to_string(),
                69,
            ),
        ];
        for (a, b, expected) in test_tuples {
            assert_eq!(min_characters(a, b), expected);
        }
    }
}
