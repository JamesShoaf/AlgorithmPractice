/*
There is an integer array perm that is a permutation of the first n positive integers,
where n is always odd.

It was encoded into another integer array encoded of length n - 1, such that
encoded[i] = perm[i] XOR perm[i + 1]. For example, if perm = [1,3,2], then encoded = [2,1].

Given the encoded array, return the original array perm. It is guaranteed that the answer
exists and is unique.
*/

// O(n) - XOR every other element of the encoded array against the XOR of 1 to n
// to find the first element, then decode.
pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
    let n = encoded.len() as i32 + 1;
    let xor_perm = (1..n + 1).fold(0, |acc, i| acc ^ i);
    let xor_after_first = (1..encoded.len())
        .step_by(2)
        .fold(0, |acc, i| acc ^ encoded[i]);
    let mut res = vec![xor_perm ^ xor_after_first];
    for num in encoded {
        res.push(*res.last().unwrap() ^ num);
    }
    res
}

// brute force - O(n^2) - try adding numbers to the result vector until a duplicate or
// out of bounds number is found
// use std::collections::HashSet;

// pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
//     let n = encoded.len() as i32 + 1;
//     let mut set = HashSet::new();
//     for i in 1..n + 1 {
//         let mut last = i;
//         set.clear();
//         set.insert(last);
//         let mut res = vec![last];
//         for &num in &encoded {
//             last ^= num;
//             if last < 1 || last > n || set.contains(&last) {
//                 break;
//             }
//             res.push(last);
//             set.insert(last);
//         }
//         if res.len() as i32 == n {
//             return res;
//         }
//     }
//     return Vec::new();
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            // (vec![3, 1], vec![1, 2, 3]),
            (vec![6, 5, 4, 6], vec![2, 4, 1, 5, 3]),
        ];
        for (encoded, expected) in test_tuples {
            assert_eq!(decode(encoded), expected);
        }
    }
}
