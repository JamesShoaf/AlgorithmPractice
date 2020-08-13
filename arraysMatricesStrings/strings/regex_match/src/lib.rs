use std::collections::HashMap;

fn regex_helper(s: &Vec<char>, p: &Vec<char>, index_s: usize, index_p: usize,
    memo: &mut HashMap<(usize, usize), bool>) -> bool {
        if let Some(prev_result) = memo.get(&(index_s, index_p)) { return *prev_result; }
        if index_p < p.len() {
            let p_char: char = p[index_p];
            // edge case - regex starts with a * or contains a **
            if p_char == '*' {
                memo.insert((index_s, index_p), false);
                return false;
            }
            let valid_s = (index_s < s.len() && s[index_s] == p_char) || p_char == '.';
            if index_p < p.len() - 1 && p[index_p + 1] == '*' {
                if regex_helper(s, p, index_s, index_p + 2, memo) { return true; }
                if valid_s {
                    if regex_helper(s, p, index_s + 1, index_p, memo) { return true; }
                    if regex_helper(s, p, index_s + 1, index_p + 2, memo) { return true; }
                }
            } else if valid_s {
                if regex_helper(s, p, index_s + 1, index_p + 1, memo) { return true; }
            }
        }
        memo.insert((index_s, index_p), false);
        false
    }

fn is_match(s: String, p: String) -> bool {
    let s = s.chars();
    let s: Vec<char> = s.collect();
    let p = p.chars();
    let p: Vec<char> = p.collect();
    if s.len() == 0 && p.len() == 0 { return true; }
    if p.len() == 0 { return false; }
    let mut memo: HashMap<(usize, usize), bool> = HashMap::new();
    // base case: both s and p have reached their final indexes
    memo.insert((s.len(), p.len()), true);
    return regex_helper(&s, &p, 0, 0, &mut memo);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::is_match(String::from(""), String::from(".*"));
        let test_tuples = vec![
            (String::from(""), String::from(""), true),
            (String::from(""), String::from("a"), false),
            (String::from(""), String::from(".*"), true),
            (String::from("a"), String::from(""), false),
            (String::from("aa"), String::from("a"), false),
            (String::from("aa"), String::from("a*"), true),
            (String::from("a"), String::from("ab*"), true),
            (String::from("aa"), String::from("a*a*a*aa"), true),
            (String::from("aa"), String::from("a*aa*a*a"), true),
            (String::from("aa"), String::from("a*aa*aa*a"), false),
            (String::from("aa"), String::from("a*a*a*aaa"), false),
            (String::from("aaa"), String::from("a**"), false),
            (String::from("ab"), String::from(".*"), true),
            (String::from("aab"), String::from("c*a*b"), true),
            (String::from("mississippi"), String::from("mis*is*p*."), false),
        ];
        for (s, p, expected) in test_tuples {
            assert_eq!(super::is_match(s.clone(), p.clone()), expected, "s: {}, p: {}", s, p);
        }
    }
}
