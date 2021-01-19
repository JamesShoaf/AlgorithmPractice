/*
find the longest substring of consecutive characters that can be rearranged to form a palindrome
*/
fn longest_palindrome_sequence(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    use std::collections::HashMap;
    let mut prev_mask_indices: HashMap<i64, usize> = HashMap::new();
    let mut mask: i64 = 0;
    prev_mask_indices.insert(mask, 0);
    let mut char_to_num: HashMap<char, i64> = HashMap::new();
    for (i, c) in (b'a'..=b'z').enumerate() {
        char_to_num.insert(c as char, i as i64);
    }
    for (i, c) in (b'A'..=b'Z').enumerate() {
        char_to_num.insert(c as char, (i + 26) as i64);
    }
    use std::cmp;
    let s = s.chars();
    let mut longest: usize = 1;
    for (i, c) in s.enumerate() {
        mask ^= 1 << *char_to_num.get(&c).unwrap();
        longest = cmp::max(
            longest,
            i + 1 - *prev_mask_indices.entry(mask).or_insert(i + 1),
        );
        for bit in 0..52 {
            if let Some(j) = prev_mask_indices.get(&(mask ^ 1 << bit)) {
                longest = cmp::max(longest, i + 1 - j);
            }
        }
    }
    longest as i32
}

fn longest_palindrome(s: String) -> i32 {
    use std::collections::HashMap;
    let mut longest = 0;
    let mut counts: HashMap<char, i32> = HashMap::new();
    let mut odd_count = 0;
    let s = s.chars();
    for c in s {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
        if *count % 2 == 0 {
            odd_count -= 1;
            longest += 2;
        } else {
            odd_count += 1;
        }
    }
    if odd_count > 0 {
        longest += 1;
    }
    longest
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sequential() {
        let test_tuples = vec![
            (String::from(""), 0),
            (String::from("hello"), 3),
            (String::from("abccccdd"), 7),
            (String::from("ABCCCCDD"), 7),
            (String::from("ABCccCDD"), 7),
            (String::from("ABCcCCDD"), 5),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(super::longest_palindrome_sequence(s), expected);
        }
    }

    #[test]
    fn test_nonsequential() {
        let test_tuples = vec![
            (String::from(""), 0),
            (String::from("a"), 1),
            (String::from("Aa"), 1),
            (String::from("abc"), 1),
            (String::from("aba"), 3),
            (String::from("abcba"), 5),
            (String::from("abcbac"), 6),
            (String::from("racecar"), 7),
            (String::from("rAaBcCeDcEaFr"), 7),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(super::longest_palindrome(s), expected);
        }
    }
}
