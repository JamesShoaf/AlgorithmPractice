/*
For a string sequence, a string word is k-repeating if word concatenated k times is a substring of
sequence. The word's maximum k-repeating value is the highest value k where word is k-repeating in
sequence. If word is not a substring of sequence, word's maximum k-repeating value is 0.

Given strings sequence and word, return the maximum k-repeating value of word in sequence.
*/

pub fn max_repeating(sequence: String, word: String) -> i32 {
    let sequence: Vec<char> = sequence.chars().collect();
    let word: Vec<char> = word.chars().collect();
    let mut max = 0;
    for i in 0..word.len() {
        let mut current = 0;
        for j in (i..sequence.len()).step_by(word.len()) {
            if j + word.len() <= sequence.len() && sequence[j..j + word.len()] == word[..] {
                current += 1;
                if current > max {
                    max = current;
                }
            } else {
                current = 0;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("ababc"), String::from("ab"), 2),
            (String::from("ababc"), String::from("ba"), 1),
            (String::from("ababc"), String::from("ac"), 0),
        ];
        for (sequence, word, expected) in test_tuples {
            assert_eq!(max_repeating(sequence, word), expected);
        }
    }
}
