/*
You are given two strings word1 and word2. You want to construct a string merge
in the following way: while either word1 or word2 are non-empty, choose one of
the following options:

    If word1 is non-empty, append the first character in word1 to merge and delete
    it from word1.
        For example, if word1 = "abc" and merge = "dv", then after choosing this
        operation, word1 = "bc" and merge = "dva".
    If word2 is non-empty, append the first character in word2 to merge and delete it from word2.
        For example, if word2 = "abc" and merge = "", then after choosing this
        operation, word2 = "bc" and merge = "a".

Return the lexicographically largest merge you can construct.

A string a is lexicographically larger than a string b (of the same length) if in the
first position where a and b differ, a has a character strictly larger than the
corresponding character in b. For example, "abcd" is lexicographically larger than
"abcc" because the first position they differ is at the fourth character, and d is greater than c.
*/

pub fn largest_merge(word1: String, word2: String) -> String {
    let chars_1 = word1.chars().collect::<Vec<char>>();
    let chars_2 = word2.chars().collect::<Vec<char>>();
    let mut res: Vec<char> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < chars_1.len() || j < chars_2.len() {
        if &chars_1[i..] > &chars_2[j..] {
            res.push(chars_1[i]);
            i += 1;
        } else {
            res.push(chars_2[j]);
            j += 1;
        }
    }
    res.into_iter().collect()
}

// use std::cmp;
// use std::collections::HashMap;

// fn recursive(
//     i: usize,
//     j: usize,
//     chars_1: &Vec<u8>,
//     chars_2: &Vec<u8>,
//     memo: &mut HashMap<(usize, usize), Vec<u8>>,
// ) -> Vec<u8> {
//     if i == chars_1.len() && j == chars_2.len() {
//         return Vec::new();
//     }
//     if let Some(prev) = memo.get(&(i, j)) {
//         return prev.clone();
//     }
//     let mut res = Vec::new();
//     if i == chars_1.len() || j < chars_2.len() && chars_1[i] < chars_2[j] {
//         res.push(chars_2[j]);
//         res.extend(recursive(i, j + 1, chars_1, chars_2, memo));
//     } else if j == chars_2.len() || chars_2[j] < chars_1[i] {
//         res.push(chars_1[i]);
//         res.extend(recursive(i + 1, j, chars_1, chars_2, memo));
//     } else if chars_1[i] == chars_2[j] {
//         res.push(chars_1[i]);
//         res.extend(cmp::max(
//             recursive(i + 1, j, chars_1, chars_2, memo),
//             recursive(i, j + 1, chars_1, chars_2, memo),
//         ));
//     }
//     memo.insert((i, j), res.clone());
//     res
// }

// pub fn largest_merge(word1: String, word2: String) -> String {
//     let chars_1 = word1.chars().map(|c| c as u8).collect::<Vec<u8>>();
//     let chars_2 = word2.chars().map(|c| c as u8).collect::<Vec<u8>>();
//     let mut memo: HashMap<(usize, usize), Vec<u8>> = HashMap::new();
//     recursive(0, 0, &chars_1, &chars_2, &mut memo)
//         .into_iter()
//         .map(|num| num as char)
//         .collect()
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
