use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let count_i = f
        .lines()
        .filter(|line| {
            if let Ok(line) = line {
                return password_validator_i(line);
            }
            false
        })
        .count();
    println!("Passwords: {}", count_i);

    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let count_ii = f
        .lines()
        .filter(|line| {
            if let Ok(line) = line {
                return password_validator_ii(line);
            }
            false
        })
        .count();
    println!("Passwords: {}", count_ii);
}

// 6-10 p: ctpppjmdpppppp - the character 'p' must occur between 6 and 10 times
fn password_validator_i(line: &str) -> bool {
    let split: Vec<&str> = line.split_whitespace().collect();
    let nums: Vec<&str> = split[0].split('-').collect();
    let min = nums[0].parse::<usize>().unwrap();
    let max = nums[1].parse::<usize>().unwrap();
    let limited_char = split[1].chars().next().unwrap();
    let mut counter = 0;
    for c in split[2].chars() {
        if c == limited_char {
            counter += 1;
        }
    }
    counter >= min && counter <= max
}

// 6-10 p: ctpppjmdpppppp - the character 'p' must occur at index 6 OR index 10 (1-indexed)
fn password_validator_ii(line: &str) -> bool {
    let split: Vec<&str> = line.split_whitespace().collect();
    let nums: Vec<&str> = split[0].split('-').collect();
    let pos1 = nums[0].parse::<usize>().unwrap();
    let pos2 = nums[1].parse::<usize>().unwrap();
    let limited_char = split[1].chars().next().unwrap();
    let mut res = false;
    for (i, c) in split[2].chars().enumerate() {
        if (i + 1 == pos1 || i + 1 == pos2) && c == limited_char {
            res = !res;
        }
    }
    res
}

mod tests {
    #[test]
    fn test_password_validator_i() {
        let test_tuples = vec![
            (String::from("6-10 p: ctpppjmdpppppp"), true),
            (String::from("17-19 l: llllllllllllllllllll"), false),
            (String::from("14-19 z: zrzzzzzztzzzzwzzzzk"), true),
            (String::from("1-8 k: qkkkkkkxkkkkkkkkk"), false),
        ];
        for (line, expected) in test_tuples {
            assert_eq!(super::password_validator_i(&line), expected);
        }
    }
    #[test]
    fn test_password_validator_ii() {
        let test_tuples = vec![
            (String::from("6-10 p: ctpppjmdpppppp"), true),
            (String::from("17-19 l: llllllllllllllllllll"), false),
            (String::from("14-19 z: zrzzzzzztzzzzwzzzzk"), false),
            (String::from("1-8 k: qkkkkkkxkkkkkkkkk"), false),
        ];
        for (line, expected) in test_tuples {
            assert_eq!(super::password_validator_ii(&line), expected);
        }
    }
}
