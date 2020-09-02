mod time_from_digits {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        if a.len() != 4 { return String::from(""); }
        let mut counts = [0; 10];
        for num in a.iter() {
            if *num > 9 || *num < 0 { return String::from(""); }
            counts[*num as usize] += 1;
        }
        if let Some(time) = choose_hour_10s(&mut counts) {
            return time;
        }
        String::from("")
    }

    fn choose_hour_10s(counts: &mut [i32; 10]) -> Option<String> {
        for i in (0..3).rev() {
            if counts[i] > 0 {
                counts[i] -= 1;
                if let Some(rest) = choose_hour_1s(i, counts) {
                    return Some(format!("{}{}", i, rest));
                }
                counts[i] += 1;
            }
        }
        None
    }
    
    fn choose_hour_1s(tens: usize, counts: &mut [i32; 10]) -> Option<String> {
        let upper = if tens == 2 { 4 } else { 10 };
        for i in (0..upper).rev() {
            if counts[i] > 0 {
                counts[i] -= 1;
                if let Some(rest) = choose_minute_10s(counts) {
                    return Some(format!("{}:{}", i, rest));
                }
                counts[i] += 1;
            }
        }
        None
    }
    
    fn choose_minute_10s(counts: &mut [i32; 10]) -> Option<String> {
        for i in (0..6).rev() {
            if counts[i] > 0 {
                counts[i] -= 1;
                if let Some(rest) = choose_minute_1s(counts) {
                    return Some(format!("{}{}", i, rest));
                }
                counts[i] += 1;
            }
        }
        None
    }
    
    fn choose_minute_1s(counts: &mut [i32; 10]) -> Option<String> {
        for i in (0..10).rev() {
            if counts[i] > 0 {
                return Some(format!("{}", i));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::time_from_digits::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), String::from("")),
            (vec![5, 5, 5, 5], String::from("")),
            (vec![2, 4, 6, 9], String::from("")),
            (vec![1, 2, 3, 4], String::from("23:41")),
            (vec![1, 9, 9, 5], String::from("19:59")),
            (vec![3, 9, 5, 0], String::from("09:53")),
            (vec![2, 4, 5, 0], String::from("20:54")),
            (vec![2, 6, 1, 9], String::from("19:26")),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(largest_time_from_digits(nums), expected);
        }
    }
}
