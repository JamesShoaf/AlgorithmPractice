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

/* 
There are a few cases to handle for this problem
if a is 0, no changes can be made to any number. Rotation only.
if a is 1, 3, 7, or 9, any number at an odd index can be changed to any other number.
if a is 2, 4, 6, or 8, any number at an odd index be changed to any number of the same parity
if a is 5, any number at an odd index can be changed only to a number 5 away.

if b is 0, no rotations can be made.
if b is odd, any rotation can be made.
if b is even and nonzero, any rotation to an even index can be made (s is of even length), unless
s's length is evenly divisible by b
*/

/* 
    2 <= s.length <= 100
    s.length is even.
    s consists of digits from 0 to 9 only.
    1 <= a <= 9
    1 <= b <= s.length - 1
*/
// given the tiny string length, a brute force solution seems feasible here

// 
use std::collections::{ HashSet, HashMap };

fn permute_rotations(mut num: String, b: i32, set: &mut HashSet<String>) {
    let mut temp = num.clone();
    let mut cycle = num.chars().cycle();
    let offset = b as usize % num.len();
    while !set.contains(&temp) {
        set.insert(temp.clone());
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
fn permute_increments(a: i32, rotated_nums: HashSet<String>) -> HashSet<String> {
    let mut increment: HashMap<char, char> = HashMap::new();
    for i in 0..10 {
        increment.insert(i as char, ((i + a as u8) % 10) as char);
    }
    let increment_limit = if a % 5 == 0 { 2 }
        else if a % 2 == 0 { 5 }
        else { 10 };
    rotated_nums.into_iter().fold(HashSet::new(), |mut acc, mut num| {
        for _ in 0..increment_limit {
            acc.insert(num.clone());
            num = num.chars().fold(String::new(), |mut acc, c| {
                acc.push(*increment.get(&c).unwrap());
                acc
            });
        }
        acc
    })
}

fn find_lex_smallest_string(s:String, a: i32, b: i32) -> String {
    let mut rotated_nums: HashSet<String> = HashSet::new();
    permute_rotations(s, b, &mut rotated_nums);
    let incremented_nums = permute_increments(a, rotated_nums);
    let mut incremented_nums: Vec<String> = incremented_nums.into_iter().collect();
    incremented_nums.sort();
    incremented_nums[0].clone()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
