struct Solution {}

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut index: usize = 0;
        let mut int: i32 = 1;
        let mut missing: i32 = 0;
        while index < arr.len() {
            if arr[index] == int { index += 1; }
            else { missing += 1; }
            if missing == k { return int; }
            int += 1;
        }
        k - missing + int - 1
    }
}

fn main() {
    let test = vec![2, 3, 4, 7, 11];
    let k = 5;
    println!("{}", Solution::find_kth_positive(test, k));
}
