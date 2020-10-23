pub mod time_from_digits {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        let mut counts = [0; 10];
        for &num in a.iter().filter(|&&x| x >= 0 && x < 10) {
            counts[num as usize] += 1;
        }
        // times contain 4 digits, and 0 as the imaginary previous digit is arbitrary
        let init = NextOptions::new(4, 0);
        if let Some(time) = choose_next(init, &mut counts) {
            return time;
        }
        String::from("")
    }

    struct NextOptions {
        max_val: usize,
        spacer: String,
        remaining: usize,
    }

    impl NextOptions {
        fn new(remaining: usize, last: usize) -> NextOptions {
            let spacer = String::from(
                if remaining == 3 { ":" }
                else { "" }
            );
            let max_val = match remaining {
                2 => 5,
                3 if last == 2 => 3,
                4 => 2,
                _ => 9,
            };
            NextOptions {
                max_val,
                spacer,
                remaining,
            }
        }
    }

    fn choose_next(options: NextOptions, counts: &mut [i32; 10]) -> Option<String> {
        if options.remaining == 0 { return Some(String::from("")); }
        for digit in (0..=options.max_val).rev() {
            if counts[digit] > 0 {
                counts[digit] -= 1;
                let init = NextOptions::new(options.remaining - 1, digit);
                if let Some(next) = choose_next(init, counts) {   
                    return Some(format!("{}{}{}", digit, options.spacer, next));
                }
                counts[digit] += 1;
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
