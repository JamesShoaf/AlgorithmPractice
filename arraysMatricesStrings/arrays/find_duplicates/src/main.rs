struct Solution {}

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            let j = (nums[i].abs() - 1) as usize;
            if nums[j] > 0 { nums[j] *= -1; }
            else { output.push(nums[i].abs()); }
        }
        output
    }
}

fn main() {
    let test = vec![2, 2, 3, 4, 5, 3];
    let output = Solution::find_duplicates(test);
    println!("{:?}", output);
}
