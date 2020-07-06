struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut reversed = digits.clone();
        reversed.reverse();
        let mut carry_bit = true;
        let length = reversed.len();
        let mut counter = 0;

        while carry_bit && counter < length {
            let total = reversed[counter] + 1;
            if total == 10 { reversed[counter] = 0; }
            else {
                reversed[counter] = total;
                carry_bit = false;
            }
            counter += 1;
        }
        if carry_bit { reversed.push(1); }
        reversed.reverse();
        reversed
    }
}

fn main() {
    let digits: Vec<i32> = vec![9, 9, 9, 9, 9, 9];
    let plus_one = Solution::plus_one(digits);
    for num in plus_one {
        println!("{}", num);
    }
}
