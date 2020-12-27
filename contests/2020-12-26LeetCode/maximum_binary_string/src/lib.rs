/*
You are given a binary string binary consisting of only 0's or 1's. You can apply each of the
following operations any number of times:

    Operation 1: If the number contains the substring "00", you can replace it with "10".
        For example, "00010" -> "10010"
    Operation 2: If the number contains the substring "10", you can replace it with "01".
        For example, "00010" -> "00001"

Return the maximum binary string you can obtain after any number of operations. Binary string x is
greater than binary string y if x's decimal representation is greater than y's decimal
representation.
*/

pub fn maximum_binary_string(binary: String) -> String {
    let chars: Vec<char> = binary.chars().collect();
    let len = chars.len();
    if len < 2 {
        return binary;
    }
    let mut leading_ones = 0;
    while leading_ones < len && chars[leading_ones] == '1' {
        leading_ones += 1;
    }
    if leading_ones == len {
        return binary;
    }
    let trailing_ones = chars[leading_ones..].iter().filter(|&&c| c == '1').count();
    let mut res = String::new();
    for _ in 0..len - trailing_ones - 1 {
        res.push('1');
    }
    res.push('0');
    for _ in 0..trailing_ones {
        res.push('1');
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
