struct Solution {}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut chars = word.chars();
        let mut initial_capital = false;
        if let Some(character) = chars.next() { initial_capital = character.is_uppercase(); }
        let mut all_caps = false;
        if let Some(character) = chars.next() {
            if character.is_uppercase() { 
                if !initial_capital { return false; }
                all_caps = true;
            }
        }
        for character in chars {
            if character.is_uppercase() != all_caps { return false; }
        }
        true
    }
}


fn main() {
    let test_tuples = vec![
        (String::new(), true),
        (String::from("😍😎🎉"), true),
        (String::from("USA"), true),
        (String::from("flag"), true),
        (String::from("Flag"), true),
        (String::from("fLag"), false),
        (String::from("FlaG"), false),
    ];
    for (word, expected) in test_tuples {
        assert_eq!(Solution::detect_capital_use(word), expected);
    }
}
