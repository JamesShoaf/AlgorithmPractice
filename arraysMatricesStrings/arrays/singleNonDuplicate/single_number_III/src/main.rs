struct Solution {}

impl Solution {
    // given an array of integers in which all but two are included twice,
    // output the two single integers
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let a_xor_b = nums.iter().fold(0, |acc, x| acc ^ x);
        // and-ing a number with its twos complement returns the lowest 1 bit
        let low_bit = a_xor_b & -a_xor_b;
        let ans = nums.iter().fold((0, 0), |mut acc, val| {
            if val & low_bit == 0 { acc.0 ^= val; }
            else {acc.1 ^= val;}
            acc
        });
        vec![ans.0, ans.1]
    }
}

fn main() {
    let nums = vec![1, 1, 2, 2, 3, 3, 4, 5];
    let ans = Solution::single_number(nums);
    println!("{:#?}", ans);
}
