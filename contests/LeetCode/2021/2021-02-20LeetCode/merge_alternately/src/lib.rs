/*
You are given two strings word1 and word2. Merge the strings by adding letters in alternating order,
starting with word1. If a string is longer than the other, append the additional letters onto the
end of the merged string.

Return the merged string.
*/

pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut res = String::new();
    let chars1: Vec<char> = word1.chars().collect();
    let chars2: Vec<char> = word2.chars().collect();
    let mut i = 0;
    while i < chars1.len() || i < chars2.len() {
        if i < chars1.len() {
            res.push(chars1[i]);
        }
        if i < chars2.len() {
            res.push(chars2[i]);
        }
        i += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
