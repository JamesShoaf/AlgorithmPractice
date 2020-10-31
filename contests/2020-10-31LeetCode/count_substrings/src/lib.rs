pub fn count_substrings(s: String, t: String) -> i32 {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let mut count = 0;
    for i in 0..s_chars.len() {
        for j in 0..t_chars.len() {
            let mut single_error = false;
            for k in 0..s_chars.len() - i {
                if j + k >= t_chars.len() { break; }
                if s_chars[i + k] != t_chars[j + k] {
                    if single_error { break; }
                    else {
                        single_error = true;
                        count += 1;
                    }
                }
                else if single_error {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("aba"), String::from("baba"), 6),
        ];
        for (s, t, expected) in test_tuples {
            assert_eq!(count_substrings(s, t), expected);
        }
    }
}
