// Design an Iterator class, which has:

//     A constructor that takes a string characters of sorted distinct lowercase English letters and a number combinationLength as arguments.
//     A function next() that returns the next combination of length combinationLength in lexicographical order.
//     A function hasNext() that returns True if and only if there exists a next combination.

struct CombinationIterator {
    chars: Vec<char>,
    indices: Vec<usize>,
}

impl CombinationIterator {
    fn new(s: String, len: i32) -> Self {
        let chars = s.chars();
        let chars: Vec<char> = chars.collect();
        let mut indices: Vec<usize> = Vec::new();
        for i in 0..len { indices.push(i as usize); }
        CombinationIterator {
            chars,
            indices,
        }
    }
    fn next(&mut self) -> String {
        let mut output: String = String::from("");
        let len = self.indices.len();
        let s_len = self.chars.len();
        if self.has_next() {
            let mut combination: Vec<char> = Vec::new();
            for i in &self.indices { combination.push(self.chars[*i]); }
            output = combination.into_iter().collect()
        }
        let mut stack: Vec<usize> = Vec::new();
        let mut index = len - 1;
        loop {
            self.indices[index] += 1;
            if self.indices[index] >= s_len - (len - 1 - index) && index > 0 {
                stack.push(index);
                index -= 1;
            } else { break; }
        }
        while !stack.is_empty() {
            let index = stack.pop().unwrap();
            self.indices[index] = self.indices[index - 1] + 1;
        }
        output
    }

    fn has_next(&self) -> bool {
        self.indices[self.indices.len() - 1] < self.chars.len()
    }
}


#[cfg(test)]
mod tests {
    use super::CombinationIterator;
    #[test]
    fn correct_order() {
        let mut iter = CombinationIterator::new(String::from("abc"), 1);
        assert_eq!(iter.next(), String::from("a"));
        assert_eq!(iter.next(), String::from("b"));
        assert_eq!(iter.next(), String::from("c"));
        assert_eq!(iter.next(), String::from(""));
        let mut iter = CombinationIterator::new(String::from("abc"), 2);
        assert_eq!(iter.next(), String::from("ab"));
        assert_eq!(iter.next(), String::from("ac"));
        assert_eq!(iter.next(), String::from("bc"));
        assert_eq!(iter.next(), String::from(""));
        let mut iter = CombinationIterator::new(String::from("abc"), 3);
        assert_eq!(iter.next(), String::from("abc"));
        assert_eq!(iter.next(), String::from(""));
    }

    #[test]
    fn has_next() {
        let mut iter = CombinationIterator::new(String::from("abc"), 1);
    }
}
