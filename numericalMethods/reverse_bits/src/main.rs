struct Solution {}

impl Solution {
    pub fn reverse_bits(num: u32) -> u32 {
        let mut output: u32 = 0;
        for index in 0..32 {
            let bit = (num & (1 << index)) >> index;
            output += (1 << (31 - index)) * bit;
        }
        output
    }
}

fn main() {
    println!("{}", Solution::reverse_bits(15));
}
