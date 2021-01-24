/*
You are given a string time in the form of hh:mm, where some of the digits in the
string are hidden (represented by ?).

The valid times are those inclusively between 00:00 and 23:59.

Return the latest valid time you can get from time by replacing the hidden digits.
*/

pub fn maximum_time(time: String) -> String {
    let mut chars: Vec<char> = time.chars().collect();
    if chars[0] == '?' {
        let ones = chars[1].to_digit(10).unwrap_or(0);
        chars[0] = if ones > 3 { '1' } else { '2' };
    }
    if chars[1] == '?' {
        chars[1] = if chars[0] == '2' { '3' } else { '9' };
    }
    chars[2] = ':';
    if chars[3] == '?' {
        chars[3] = '5';
    }
    if chars[4] == '?' {
        chars[4] = '9';
    }
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
