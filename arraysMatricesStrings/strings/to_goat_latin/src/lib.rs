pub fn to_goat_latin(s: String) -> String {
    use std::collections::HashSet;
    let mut vowels: HashSet<char> = HashSet::new();
    for vowel in vec!['a', 'e', 'i', 'o', 'u'] {
        vowels.insert(vowel.to_uppercase().next().unwrap());
        vowels.insert(vowel);
    }
    let mut output = String::new();
    for (i, word) in s.split_ascii_whitespace().enumerate() {
        if i > 0 { output.push(' '); }
        let mut chars = word.chars();
        let first = chars.next().unwrap();
        let rest = chars.collect::<String>();
        if vowels.contains(&first) {
            output.push(first);
            output.push_str(&rest);
        } else {
            output.push_str(&rest);
            output.push(first);
        }
        output.push_str("maa");
        for _ in 0..i { output.push('a'); }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            ("I speak Goat Latin", "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"),
            ("The quick brown fox jumped over the lazy dog", "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(to_goat_latin(String::from(s)), expected);
        }
    }
}
