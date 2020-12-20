/*
An encoded string S is given.  To find and write the decoded string to a tape, the encoded string
is read one character at a time and the following steps are taken:

    If the character read is a letter, that letter is written onto the tape.
    If the character read is a digit (say d), the entire current tape is repeatedly written d-1
    more times in total.

Now for some encoded string S, and an index K, find and return the K-th letter (1 indexed) in the
decoded string.
*/

use std::collections::HashMap;

pub fn decode_at_index(s: String, k: i32) -> String {
    let mut k = k as u64 - 1;
    let mut len: u64 = 0;
    let mut map: HashMap<u64, char> = HashMap::new();
    let mut multiply_stack: Vec<u64> = Vec::new();
    for c in s.chars() {
        if len > k {
            break;
        }
        if let Some(num) = c.to_digit(10) {
            multiply_stack.push(len);
            if let Some(next) = len.checked_mul(num as u64) {
                len = next;
            } else {
                break;
            }
        } else {
            if k == len {
                return String::from(c);
            }
            map.insert(len, c);
            len += 1;
        }
    }
    while !map.contains_key(&k) {
        k %= multiply_stack.pop().unwrap();
    }
    String::from(*map.get(&k).unwrap())
}

// initial recursive implementation - fails for very complex cases
// fn recursive_decode(chars: &Vec<char>, k: u32) -> char {
//     let mut len = 0;
//     for i in 0..chars.len() {
//         if let Some(num) = chars[i].to_digit(10) {
//             let total_len = len * num;
//             if k <= total_len {
//                 if k % len == 0 {
//                     return (0..=i)
//                         .rev()
//                         .map(|c| chars[c])
//                         .find(|c| !c.is_digit(10))
//                         .unwrap();
//                 }
//                 return recursive_decode(chars, k % len);
//             }
//             len = total_len;
//         } else {
//             len += 1;
//             if k == len {
//                 return chars[i];
//             }
//         }
//     }
//     // if k == 0 or k > len
//     '❌'
// }

// pub fn decode_at_index(s: String, k: i32) -> String {
//     let k = k as usize;
//     let chars = s.chars().collect();
//     String::from(recursive_decode(&chars, k as u32))
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("abcd2efgh3"), 1, 'a'),
            (String::from("abcd2efgh3"), 2, 'b'),
            (String::from("abcd2efgh3"), 3, 'c'),
            (String::from("abcd2efgh3"), 4, 'd'),
            (String::from("abcd2efgh3"), 5, 'a'),
            (String::from("abcd2efgh3"), 6, 'b'),
            (String::from("abcd2efgh3"), 7, 'c'),
            (String::from("abcd2efgh3"), 8, 'd'),
            (String::from("abcd2efgh3"), 9, 'e'),
            (String::from("abcd2efgh3"), 10, 'f'),
            (String::from("abcd2efgh3"), 11, 'g'),
            (String::from("abcd2efgh3"), 12, 'h'),
            (String::from("abcd2efgh3"), 13, 'a'),
            (String::from("abcd2efgh3"), 14, 'b'),
            (String::from("abcd2efgh3"), 15, 'c'),
            (String::from("abcd2efgh3"), 16, 'd'),
            (String::from("abcd2efgh3"), 17, 'a'),
            (String::from("abcd2efgh3"), 18, 'b'),
            (String::from("abcd2efgh3"), 19, 'c'),
            (String::from("abcd2efgh3"), 20, 'd'),
            (String::from("abcd2efgh3"), 21, 'e'),
            (String::from("abcd2efgh3"), 22, 'f'),
            (String::from("abcd2efgh3"), 23, 'g'),
            (String::from("abcd2efgh3"), 24, 'h'),
            (String::from("abcd2efgh3"), 25, 'a'),
            (String::from("abcd2efgh3"), 26, 'b'),
            (String::from("abcd2efgh3"), 27, 'c'),
            (String::from("abcd2efgh3"), 28, 'd'),
            (String::from("abcd2efgh3"), 29, 'a'),
            (String::from("abcd2efgh3"), 30, 'b'),
            (String::from("abcd2efgh3"), 31, 'c'),
            (String::from("abcd2efgh3"), 32, 'd'),
            (String::from("abcd2efgh3"), 33, 'e'),
            (String::from("abcd2efgh3"), 34, 'f'),
            (String::from("abcd2efgh3"), 35, 'g'),
            (String::from("abcd2efgh3"), 36, 'h'),
            (String::from("ha22"), 1, 'h'),
            (String::from("ha22"), 2, 'a'),
            (String::from("ha22"), 3, 'h'),
            (String::from("ha22"), 4, 'a'),
            (String::from("ha22"), 5, 'h'),
            (String::from("ha22"), 6, 'a'),
            (String::from("ha22"), 7, 'h'),
            (String::from("ha22"), 8, 'a'),
            (String::from("a2345678999999999999999"), 1, 'a'),
            (String::from("a2345678999999999999999"), 2, 'a'),
            (String::from("a2345678999999999999999"), 100, 'a'),
            (String::from("a2345678999999999999999"), 10000000, 'a'),
            (String::from("ajx37nyx97niysdrzice4petvcvmcgqn282zicpbx6okybw93vhk782unctdbgmcjmbqn25rorktmu5ig2qn2y4xagtru2nehmsp"), 976159153, 'a'),
        ];
        for (s, k, expected) in test_tuples {
            assert_eq!(decode_at_index(s, k), String::from(expected));
        }
    }
}
