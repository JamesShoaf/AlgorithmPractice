use std::collections::HashMap;

fn word_pattern(pattern: String, str: String) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let str: Vec<&str> = str.split_whitespace().collect();
    if pattern.len() != str.len() { return false; }
    let mut char_to_str: HashMap<char, &str> = HashMap::new();
    let mut str_to_char: HashMap<&str, char> = HashMap::new();
    for i in 0..str.len() {
        let str_from_char = char_to_str.entry(pattern[i]).or_insert(str[i]);
        let char_from_str = str_to_char.entry(str[i]).or_insert(pattern[i]);
        if *str_from_char != str[i] || *char_from_str != pattern[i] { return false; }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("abba"), String::from("dog cat cat dog"), true),
            (String::from("aaaa"), String::from("dog dog dog dog"), true),
            (String::from("abba"), String::from("dog dog dog dog"), false),
            (String::from("aba"), String::from("dog cat cat dog"), false),
            (String::from("aaa"), String::from("dog dog dog dog"), false),
            (String::from("abcd"), String::from("dog cat cat dog"), false),
            (String::from("abba"), String::from("dog cat cat fish"), false),
        ];
        for (pattern, str, expected) in test_tuples {
            assert_eq!(word_pattern(pattern, str), expected);
        }
    }
}
