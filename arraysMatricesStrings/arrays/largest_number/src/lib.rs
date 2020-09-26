/* 
Given a list of non negative integers, arrange them such that they form the largest number.
The result may be very large, so you need to return a string instead of an integer.
*/
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct NumString {
    num: String,
}

impl NumString {
    fn new(num: i32) -> Self {
        NumString {
            num: format!("{}", num),
        }
    }
}

impl PartialOrd for NumString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NumString {
    fn cmp(&self, other: &Self) -> Ordering {
        let ab = format!("{}{}", self.num, other.num);
        let ba = format!("{}{}", other.num, self.num);
        for (a, b) in ab.chars().zip(ba.chars()) {
            let a = a.to_digit(10).unwrap();
            let b = b.to_digit(10).unwrap();
            if a > b { return Ordering::Greater; }
            if a < b { return Ordering::Less; }
        }
        Ordering::Equal
    }
}

pub fn largest_number(nums: Vec<i32>) -> String {
    let mut nums: Vec<NumString> = nums.into_iter()
        .map(|num| NumString::new(num))
        .collect();
    nums.sort();
    nums.reverse();
    nums.into_iter().fold(String::new(), |acc, val| {
        if acc == String::from("0") { val.num }
        else { acc + &val.num }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![10, 2], String::from("210")),
            (vec![3, 30, 34, 5, 9], String::from("9534330")),
            (vec![3, 30, 33, 303, 330, 333], String::from("33333333030330")),
            (vec![3], String::from("3")),
            (vec![0, 0], String::from("0")),
            (vec![0, 1], String::from("10")),
            (Vec::new(), String::from("")),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(largest_number(nums), expected);
        }
    }
}
