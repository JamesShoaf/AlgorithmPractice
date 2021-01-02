fn combine(word_array: Vec<String>) -> String {
    let mut res = String::new();
    for s in word_array {
        res.push_str(&s);
    }
    res
}

pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    combine(word1) == combine(word2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
