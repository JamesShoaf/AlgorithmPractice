struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut output: String = s.split(' ')
            .filter(|word| word.len() > 0)
            .rev()
            .fold(String::new(), |s, word| s + &word + " ");
        output.pop();
        output
    }
}

fn main() {
    let test_string = String::from("  a good   example  ");
    println!("{}", Solution::reverse_words(test_string));
}
