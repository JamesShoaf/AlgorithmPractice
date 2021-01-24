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

fn at_most_cost(counts: &Vec<(char, u32)>) -> [u32; ALPHABET_LEN] {
    let len = counts.iter().map(|&(_, count)| count).sum::<u32>();
    let mut res = [0; ALPHABET_LEN];
    let mut res_ptr = ALPHABET_LEN - 1;
    let mut counts_ptr = counts.len() as isize - 1;
    for i in (0..ALPHABET_LEN + 1).rev() {}
    res
}

fn at_least_cost(counts: &Vec<(char, u32)>) -> [u32; ALPHABET_LEN] {
    let len = counts.iter().map(|&(_, count)| count).sum::<u32>();
    let mut res = [0; ALPHABET_LEN];
    let mut counts_ptr = 0;
    for i in 0..ALPHABET_LEN + 1 {}
    res
}

fn a_greater_cost(a: &[u32; ALPHABET_LEN], b: &[u32; ALPHABET_LEN], res: &mut u32) {
    for i in 0..25 {
        *res = cmp::min(*res, a[i + 1] + b[i]);
    }
}

fn single_char_cost(counts: &Vec<(char, u32)>) -> u32 {
    let len = counts.iter().map(|&(_, count)| count).sum::<u32>();
    let max = *counts.iter().map(|(_, count)| count).max().unwrap_or(&0);
    len - max
}

pub fn min_characters(a: String, b: String) -> i32 {
    let a_counts = get_counts(&a);
    let b_counts = get_counts(&b);
    let mut res = single_char_cost(&a_counts) + single_char_cost(&b_counts);
    let a_lower = at_least_cost(&a_counts);
    let a_raise = at_most_cost(&a_counts);
    let b_lower = at_least_cost(&b_counts);
    let b_raise = at_most_cost(&b_counts);
    a_greater_cost(&a_raise, &b_lower, &mut res);
    a_greater_cost(&b_raise, &a_lower, &mut res);
    res as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
