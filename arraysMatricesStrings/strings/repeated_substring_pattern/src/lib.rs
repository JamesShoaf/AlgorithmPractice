fn repeated_substring_pattern(s: String) -> bool {
    let s = s.as_bytes();
    (1..=s.len()/2).filter(|&x| s.len() % x == 0).any(|x| -> bool {
        let mut chunks = s.chunks_exact(x);
        let first = chunks.next().unwrap();
        chunks.all(|x| x == first)
    })
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
