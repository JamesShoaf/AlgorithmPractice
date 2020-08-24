use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
// #[derive(Debug, Deref, DerefMut)]
struct Trie {
    ends: bool,
    // #[deref]
    // #[deref_mut]
    root: Rc<RefCell<HashMap<char, Trie>>>,
}

impl Deref for Trie {
    type Target = Rc<RefCell<HashMap<char, Trie>>>;
    fn deref(&self) -> &Self::Target { &self.root }
}

impl DerefMut for Trie {
    // type Target = Rc<RefCell<HashMap<char, Trie>>>;
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.root }
}

// impl DerefMut for Trie {

// }

impl Trie {
    fn new () -> Self {
        Trie {
            root: Rc::new(RefCell::new(HashMap::new())),
            ends: false,
        }
    }

    fn insert(&mut self, s: String) {
        let mut current = self;
        for c in s.chars() {
            current = current.root.get_mut().entry(c).or_insert(Trie::new());
        }
        current.ends = true;
    }
}

#[derive(Debug)]
struct StreamChecker {
    trie: Trie,
    stream: Vec<Rc<RefCell<HashMap<char, Trie>>>>,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(word);
        }
        StreamChecker {
            trie,
            stream: Vec::new(),
        }
    }

    fn query(&mut self, c: char) -> bool {
        println!("c: {}, stream: {:?}", c, self.stream);
        let mut found = false;
        let mut next_stream: Vec<Rc<RefCell<HashMap<char, Trie>>>> = Vec::new();
        self.stream.push(self.trie.root.clone());
        for trie in self.stream.iter() {
            if let Some(trie) = trie.borrow().get(&c) {
                if trie.ends { found = true; }
                next_stream.push(trie.root.clone());
            }
        }
        self.stream = next_stream;
        found
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let words = vec![String::from("fan"), String::from("and"), String::from("of")];
        let mut checker = StreamChecker::new(words);
        let test_tuples = vec![
            ('n', false),
            ('d', false),
            ('f', false),
            ('a', false),
            ('n', true),
            ('d', true),
            ('o', false),
            ('f', true),
            ('a', false),
        ];
        for (c, expected) in test_tuples {
            assert_eq!(checker.query(c), expected, "{}", c);
        }
    }
}
