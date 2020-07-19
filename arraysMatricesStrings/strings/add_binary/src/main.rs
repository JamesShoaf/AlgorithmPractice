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
            let mut counter = 0;
            if carry_bit { counter += 1 }
            carry_bit = false;
            if let Some('1') = a_next { counter += 1}
            if let Some('1') = b_next { counter += 1}
            match counter {
                1 => output.push('1'),
                2 => {
                    carry_bit = true;
                    output.push('0');
                }
                3 => {
                    carry_bit = true;
                    output.push('1');
                }
                _ => output.push('0'),
            }
            a_next = a_chars.next();
            b_next = b_chars.next();
        }
        if carry_bit { output.push('1'); }
        output.iter().rev().collect()
    }
}

fn main() {
    let a = String::from("10001");
    let b = String::from("1111");
    let c = Solution::add_binary(a, b);
    println!("{}", c);
}
