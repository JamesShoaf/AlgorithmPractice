pub mod partition_labels {
    use std::collections::HashMap;
    use std::cmp;
    fn partition_from(start: usize, s: &Vec<char>, last_index: &HashMap<char, usize>) -> Vec<i32> {
        if start >= s.len() { return Vec::new(); }
        let mut end = start;
        let mut i = start;
        while i <= end {
            end = cmp::max(end, *last_index.get(&s[i]).unwrap());
            i += 1;
        }
        let mut output = vec![(end - start + 1) as i32];
        output.append(&mut partition_from(end + 1, s, last_index));
        output
    }
    
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_index: HashMap<char, usize> = HashMap::new();
        let s: Vec<char> = s.chars().collect();
        for (i, &c) in s.iter().enumerate() {
            last_index.insert(c, i);
        }
        let mut output: Vec<i32> = Vec::new();
        output.append(&mut partition_from(0, &s, &last_index));
        output
    }
}

#[cfg(test)]
mod tests {
    use super::partition_labels::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from(""), Vec::new()),
            (String::from("a"), vec![1]),
            (String::from("ab"), vec![1, 1]),
            (String::from("aba"), vec![3]),
            (String::from("ababcbacadefegdehijhklij"), vec![9, 7, 8]),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(partition_labels(s), expected);
        }
    }
}
