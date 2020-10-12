/* 
Given a string s, remove duplicate letters so that every letter appears once and only once. You
must make sure your result is the smallest in lexicographical order among all possible results.
*/

use::std::collections::{ HashMap, HashSet };

pub fn remove_duplicate_letters(s:String) -> String {
    let mut counts: HashMap<char, u32> = HashMap::new();
    let mut output: Vec<char> = Vec::new();
    let mut included: HashSet<char> = HashSet::new();

    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    for c in s.chars() {
        *counts.get_mut(&c).unwrap() -= 1;
        if !included.contains(&c) {
            loop {
                if output.is_empty()
                || *output.last().unwrap() < c
                || *counts.get(output.last().unwrap()).unwrap() == 0 {
                    output.push(c);
                    included.insert(c);
                    break;
                }
                included.remove(&output.pop().unwrap());
            }
        }
    }
    output.into_iter().collect()
}

// initial Peekable implementation
// pub fn remove_duplicate_letters(s: String) -> String {
//     // create a map of vectors of indexes for each character (in ascending order)
//     let mut indexes: HashMap<char, Vec<usize>> = HashMap::new();
//     for (i, c) in s.chars().enumerate() {
//         indexes.entry(c).or_insert(Vec::new()).push(i);
//     }
//     // convert that map of vectors to a map of Peeakble iterators
//     // use peek to check if the iterator is empty without advancing it
//     type Peekable = std::iter::Peekable<std::vec::IntoIter<usize>>;
//     let mut iters: HashMap<char, Peekable> = indexes.into_iter()
//         .map(|(c, vec)| (c, vec.into_iter().peekable()))
//         .collect();
//     // stack of current letters for output, set of letters in output
//     let mut output: Vec<char> = Vec::new();
//     let mut included: HashSet<char> = HashSet::new();
//     // greedily add each character to the output
//     for c in s.chars() {
//         // advance the current letter's iterator
//         iters.get_mut(&c).unwrap().next();
//         // if the current character is in the output, ignore it
//         if included.contains(&c) { continue; }
//         loop {
//             // if the current char is lexicographically after the current top of the stack,
//             // or if there are no later versions of the character (the char is locked)
//             if output.is_empty()
//             || c > output[output.len() - 1]
//             || iters.get_mut(&output[output.len() - 1]).unwrap()
//             .peek().is_none() 
//             {
//                 // insert the current character and stop iteration
//                 included.insert(c);
//                 output.push(c);
//                 break;
//             // otherwise, remove the top of the stack and continue the current loop
//             } else {
//                 included.remove(
//                     &output.pop().unwrap()
//                 );
//             }
//         }
//     }
//     output.into_iter().collect()
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::new(), String::new()),
            (String::from("bcabc"), String::from("abc")),
            (String::from("bcacb"), String::from("acb")),
            (String::from("acccbc"), String::from("abc")),
            (String::from("cbacdcbc"), String::from("acdb")),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(remove_duplicate_letters(s), expected);
        }
    }
}
