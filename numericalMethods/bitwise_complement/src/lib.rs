/* 
Every non-negative integer N has a binary representation.  For example, 5 can be represented as
"101" in binary, 11 as "1011" in binary, and so on.  Note that except for N = 0, there are no
leading zeroes in any binary representation.

The complement of a binary representation is the number in binary you get when changing every 1 to
a 0 and 0 to a 1.  For example, the complement of "101" in binary is "010" in binary.

For a given number N in base-10, return the complement of it's binary representation as a base-10
integer.
*/

pub fn bitwise_complement(num: i32) -> i32 {
    // 0 has no most significant set bit, so return 1 instead
    if num == 0 { return 1; }
    // -1 has a binary representation of 32 1s.
    let mut res = -1;
    let mut shift = 31;
    // turn off set bits in res until the most significant set bit of num is found
    while num & 1 << shift == 0 && shift >= 0 {
        res ^= 1 << shift;
        shift -= 1;
    }
    // then toggle each set bit in num off in res
    res ^ num
}

// #1 solution for comparison

// pub fn bitwise_complement(x: i32) -> i32 {
//     let mut n1 = x;
//     if(n1==0){
//         return 1;
//     }
//     let mut n=0;
//     let temp = n1;
//     while(n1 != 0){
//         n = (n << 1) + 1;
//         n1 =n1>>1;
//     }
    
//     n -temp
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (0, 1),
            (1, 0),
            (2, 1),
            (3, 0),
            (7, 0),
            (8, 7),
            (1023, 0),
            (1024, 1023),
            (-1, 0),
            (-2, 1),
        ];
        for (num, expected) in test_tuples {
            assert_eq!(bitwise_complement(num), expected);
        }
    }
}
