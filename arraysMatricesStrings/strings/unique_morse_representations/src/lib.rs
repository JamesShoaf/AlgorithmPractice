/*
International Morse Code defines a standard encoding where each letter is mapped to a series of
dots and dashes, as follows: "a" maps to ".-", "b" maps to "-...", "c" maps to "-.-.", and so on.

Now, given a list of words, each word can be written as a concatenation of the Morse code of each
letter. For example, "cab" can be written as "-.-..--...", (which is the concatenation "-.-." +
".-" + "-..."). We'll call such a concatenation, the transformation of a word.

Return the number of different transformations among all words we have.
*/

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn string_to_morse(s: String, dictionary: &HashMap<char, &str>) -> String {
    let mut res = String::new();
    for c in s.chars() {
        if let Some(morse) = dictionary.get(&c) {
            res.push_str(morse);
        }
    }
    res
}

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let morse_abecedary: Vec<&str> = vec![
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];
    let abecedary: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let dictionary: HashMap<char, &str> =
        HashMap::from_iter(abecedary.into_iter().zip(morse_abecedary.into_iter()));
    let mut morse_concatenations: HashSet<String> = HashSet::new();
    for word in words {
        morse_concatenations.insert(string_to_morse(word, &dictionary));
    }
    morse_concatenations.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![(
            vec![
                String::from("gin"),
                String::from("zen"),
                String::from("gig"),
                String::from("msg"),
            ],
            2,
        )];
        for (words, expected) in test_tuples {
            assert_eq!(unique_morse_representations(words), expected);
        }
    }
}
