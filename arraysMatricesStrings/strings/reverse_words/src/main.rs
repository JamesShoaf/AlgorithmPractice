struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

fn main() {
    let test_string = String::from("  a good   example  ");
    println!("{}", Solution::reverse_words(test_string));
}
