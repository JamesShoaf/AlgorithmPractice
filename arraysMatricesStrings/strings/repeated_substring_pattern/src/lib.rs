fn repeated_substring_pattern(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for length in 1..=s.len() / 2 {
        if s.len() % length == 0 {
            let mut length_matches = true;
            for repeat in 1..s.len() / length {
                let mut pattern_matches = true;
                for index in 0..length {
                    if chars[index] != chars[repeat * length + index] {
                        pattern_matches = false;
                        break;
                    }
                }
                if !pattern_matches {
                    length_matches = false;
                    break;
                }
            }
            if length_matches { return true; }
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from(""), false),
            (String::from("a"), false),
            (String::from("aa"), true),
            (String::from("aaaaaaaaa"), true),
            (String::from("ab"), false),
            (String::from("abab"), true),
            (String::from("abba"), false),
            (String::from("abbaabba"), true),
            (String::from("abcabcabcabc"), true),
            (String::from("abcabcabcacb"), false),
            (String::from("abaababaab"), true),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(repeated_substring_pattern(s), expected);
        }
    }
}
