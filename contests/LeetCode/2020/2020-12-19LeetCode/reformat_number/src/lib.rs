/*
You are given a phone number as a string number. number consists of digits, spaces ' ', and/or dashes '-'.

You would like to reformat the phone number in a certain manner. Firstly, remove all spaces and dashes. Then, group the digits from left to right into blocks of length 3 until there are 4 or fewer digits. The final digits are then grouped as follows:

    2 digits: A single block of length 2.
    3 digits: A single block of length 3.
    4 digits: Two blocks of length 2 each.

The blocks are then joined by dashes. Notice that the reformatting process should never produce any blocks of length 1 and produce at most two blocks of length 2.

Return the phone number after formatting.
*/

pub fn reformat_number(s: String) -> String {
    let nums: Vec<char> = s.chars().filter(|c| c.is_digit(10)).collect();
    let mut index = 0;
    let mut res = String::new();
    while nums.len() - index > 4 {
        for _ in 0..3 {
            res.push(nums[index]);
            index += 1;
        }
        res.push('-');
    }
    if nums.len() - index == 4 {
        for _ in 0..2 {
            res.push(nums[index]);
            index += 1;
        }
        res.push('-');
        for _ in 0..2 {
            res.push(nums[index]);
            index += 1;
        }
    } else {
        while index < nums.len() {
            res.push(nums[index]);
            index += 1;
        }
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
