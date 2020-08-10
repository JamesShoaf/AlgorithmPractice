struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut count: i64 = 0;
        let mut place: i64 = 1;
        let abecedary = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let mut map: HashMap<char, i64> = HashMap::new();
        for (index, chr) in abecedary.chars().enumerate() {
            map.insert(chr, (index + 1) as i64);
        }
        for chr in s.chars().rev() {
            count += map.get(&chr).unwrap() * place;
            place *= 26;
        }
        count as i32
    }
}

fn main() {
    let test_tuples = vec![
        (String::from("A"), 1),
        (String::from("B"), 2),
        (String::from("Z"), 26),
        (String::from("AA"), 27),
        (String::from("AZ"), 52),
        (String::from("ZZ"), 702),
        (String::from("FXSHRXW"), 2147483647),
    ];
    for (s, expected) in test_tuples { assert_eq!(Solution::title_to_number(s), expected); }
}
