/* 
You are given a string s of even length consisting of digits from 0 to 9, and two integers a and b.

You can apply either of the following two operations any number of times and in any order on s:

    Add a to all odd indices of s (0-indexed). Digits post 9 are cycled back to 0. For example, if
    s = "3456" and a = 5, s becomes "3951".
    Rotate s to the right by b positions. For example, if s = "3456" and b = 1, s becomes "6345".

Return the lexicographically smallest string you can obtain by applying the above operations any
number of times on s.

A string a is lexicographically smaller than a string b (of the same length) if in the first
position where a and b differ, string a has a letter that appears earlier in the alphabet than the
corresponding letter in b. For example, "0158" is lexicographically smaller than "0190" because
the first position they differ is at the third letter, and '5' comes before '9'.
*/

use std::collections::{ HashSet, VecDeque };

fn permute_rotations(num: &String, b: i32, visited: &mut HashSet<String>, queue: &mut VecDeque<String>) {
    let mut temp = num.clone();
    let mut cycle = num.chars().cycle();
    let offset = b as usize % num.len();
    while !visited.contains(&temp) {
        visited.insert(temp.clone());
        queue.push_back(temp.clone());
        for _ in 0..offset {
            cycle.next();
        }
        let mut vec: Vec<char> = Vec::new();
        for _ in 0..num.len() {
            vec.push(cycle.next().unwrap());
        }
        temp = vec.iter().collect();
    }
}

fn increment_word(s: String, a: i32, visited: &HashSet<String>, queue: &mut VecDeque<String>) {
    if a == 0 { return; }
    let mut chars: Vec<char> = s.chars().collect();
    for i in (1..chars.len()).step_by(2) {
        chars[i] = std::char::from_digit(
            (chars[i].to_digit(10).unwrap() + a as u32) % 10, 10)
            .unwrap();
    }
    let string = chars.into_iter().collect();
    if !visited.contains(&string) {
        queue.push_back(string);
    }
}

pub fn find_lex_smallest_string(s:String, a: i32, b: i32) -> String {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    permute_rotations(&s, b, &mut visited, &mut queue);
    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();
        if !visited.contains(&next) {
            permute_rotations(&next, b, &mut visited, &mut queue);
        }
        increment_word(next, a, &visited, &mut queue);
    }
    let mut permutations: Vec<String> = visited.into_iter().collect();
    permutations.sort();
    permutations[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("5525"), 9, 2, String::from("2050")),
            (String::from("74"), 5, 1, String::from("24")),
            (String::from("0011"), 4, 2, String::from("0011")),
            (String::from("43987654"), 7, 3, String::from("00553311")),
        ];
        for (s, a, b, expected) in test_tuples {
            assert_eq!(find_lex_smallest_string(s, a, b), expected);
        }
    }
}
