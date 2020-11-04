/* 
Given a string s, the power of the string is the maximum length of a non-empty substring that
contains only one unique character.

Return the power of the string.
*/

pub fn max_power(s: String) -> i32 {
    s.chars().scan((0, 'a'), |(count, p), c| {
        if *p == c {
            *count += 1;
        } else {
            *count = 1;
            *p = c;
        }
        Some(*count)
    }).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from(""), 0),
            (String::from("abcdef"), 1),
            (String::from("leetcode"), 2),
            (String::from("abbcccddddeeeeedcba"), 5),
            (String::from("triplepillooooow"), 5),
            (String::from("hooraaaaaaaaaaay"), 11),
            (String::from("tourist"), 1),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(max_power(s), expected);
        }
    }
}
