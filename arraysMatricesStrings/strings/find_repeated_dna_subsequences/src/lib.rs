/* 
All DNA is composed of a series of nucleotides abbreviated as 'A', 'C', 'G', and 'T', for example:
"ACGAATTCCG". When studying DNA, it is sometimes useful to identify repeated sequences within the
DNA.

Write a function to find all the 10-letter-long sequences (substrings) that occur more than once in
a DNA molecule.
*/

use std::collections::HashMap;

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let mut seq_map: HashMap<&[char], usize> = HashMap::new();
    let chars: Vec<char> = s.chars().collect();
    for seq in chars.windows(10) {
        *seq_map.entry(seq).or_insert(0) += 1;
    }
    seq_map.into_iter()
        .filter(|&(_, count)| count > 1)
        .fold(Vec::new(), |mut acc, (seq, _)| {
            acc.push(seq.iter().collect());
            acc 
        })
}

/* 
#1 solution for comparison
#1 time

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    if s.len() <= 10 {
        return vec![];
    }
    let mut all: HashSet<&str> = HashSet::new();
    let mut rep: HashSet<&str> = HashSet::new();
    
    let mut l = 0;
    let mut r = 10;
    
    while r <= s.len() {
        if all.contains(&s[l..r]) {
            rep.insert(&s[l..r]);
        } else {
            all.insert(&s[l..r]);
        }
        l += 1;
        r += 1;
    }
    
    Vec<String> = rep.iter().map(|i| i.to_string()).collect()
}

#1 space
pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    use std::collections::HashSet;

    if s.len() < 11 {
        return vec![];
    }
    let mut map = vec![0; 26];
    map[(b'C' - b'A') as usize] = 1;
    map[(b'G' - b'A') as usize] = 2;
    map[(b'T' - b'A') as usize] = 3;

    let mut seen = HashSet::<i32>::new();
    let mut repeated = HashSet::<i32>::new();
    let mut ans = vec![];
    for i in 0..s.len() - 9 {
        let mut v = 0;
        for j in i..i + 10 {
            v <<= 2;
            v |= map[(s.as_bytes()[j] - b'A') as usize];
        }
        let ten = s[i..i + 10].to_owned();
        if !seen.insert(v) && repeated.insert(v) {
            ans.push(s[i..i + 10].to_owned());
        }
    }
    ans
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("A"), Vec::new()),
            (String::from("ACTGACTGACTG"), Vec::new()),
            (String::from("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"),
            vec![
                String::from("AAAAACCCCC"),
                String::from("CCCCCAAAAA"),
            ]),
            (String::from("AAAAAAAAAAAAA"),
            vec![
                String::from("AAAAAAAAAA"),
            ]),
        ];
        for (s, expected) in test_tuples {
            let mut sequences = find_repeated_dna_sequences(s);
            sequences.sort();
            assert_eq!(sequences, expected);
        }
    }
}
