use std::collections::HashMap;

struct WordDictionary {
    dictionary: HashMap<char, WordDictionary>,
    ends: bool,
}

impl WordDictionary {
    fn new() -> WordDictionary {
        WordDictionary {
            dictionary: HashMap::new(),
            ends: false,
        }
    }

    fn add_word(&mut self, s: String) -> () {
        let lowercase = s.to_ascii_lowercase();
        let chars = lowercase.chars();
        let mut current = self;
        for c in chars {
            current = current.dictionary.entry(c).or_insert(WordDictionary::new());
        }
        current.ends = true;
    }

    fn search(&self, word: String) -> bool {
        let chars = word.chars().collect::<Vec<char>>();
        fn helper(trie: &WordDictionary, chars: &Vec<char>, index: usize, ) -> bool {
            if index == chars.len() { return trie.ends; }
            if chars[index] == '.' {
                let children = &trie.dictionary;
                for (_, v) in children {
                    if helper(v, chars, index + 1) { return true; }
                }
            } else if let Some(child) = trie.dictionary.get(&chars[index]) {
                if helper(child, chars, index + 1) { return true; }
            }
            false
        }
        helper(&self, &chars, 0)
    }
}


fn main() {
    let mut trie = WordDictionary::new();
    trie.add_word(String::from("foo"));
    let test_searches = vec![
        (String::from(""), false),
        (String::from("foo"), true),
        (String::from("..."), true),
        (String::from(".oo"), true),
        (String::from("f.o"), true),
        (String::from("fo."), true),
        (String::from("f.."), true),
        (String::from(".o."), true),
        (String::from("..o"), true),
        (String::from("food"), false),
        (String::from("...d"), false),
    ];
    for (string, expected) in test_searches {
        assert_eq!(trie.search(string), expected);
    }
}
