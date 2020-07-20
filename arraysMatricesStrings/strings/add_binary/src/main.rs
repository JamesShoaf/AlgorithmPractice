struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_chars = a.chars().rev();
        let mut b_chars = b.chars().rev();
        let mut a_next = a_chars.next();
        let mut b_next = b_chars.next();
        let mut output: Vec<char> = Vec::new();
        let mut carry_bit = false;
        while a_next.is_some() || b_next.is_some() {
            let mut counter = carry_bit as u32;
            if let Some('1') = a_next { counter += 1}
            if let Some('1') = b_next { counter += 1}
            output.push(std::char::from_digit(counter % 2, 2).unwrap());
            carry_bit = counter >= 2;
            a_next = a_chars.next();
            b_next = b_chars.next();
        }
        output.push(std::char::from_digit(carry_bit as u32, 2).unwrap());
        output.iter().rev().collect()
    }
}

fn main() {
    let a = String::from("10001");
    let b = String::from("1111");
    let c = Solution::add_binary(a, b);
    println!("{}", c);
}
