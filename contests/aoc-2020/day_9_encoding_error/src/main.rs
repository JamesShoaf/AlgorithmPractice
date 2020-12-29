use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

const PREAMBLE_LENGTH: usize = 25;
fn main() {
    if let Ok(f) = File::open("input.txt") {
        let f = BufReader::new(f);
        let nums: Vec<u64> = f
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|string| string.parse().ok())
            .collect();
        if let Some(invalid) = first_invalid_number(&nums) {
            println!("{}", invalid);
            if let Some(weakness) = find_encryption_weakness(invalid, &nums) {
                println!("{}", weakness);
            }
        }
    }
}

// Each number (up until the invalid number) is the sum of two of the previous PREAMBLE_LENGTH numbers
// Find the first invalid number
fn first_invalid_number(nums: &Vec<u64>) -> Option<u64> {
    if nums.len() <= PREAMBLE_LENGTH {
        return None;
    }
    let mut counts = HashMap::new();
    for i in 0..PREAMBLE_LENGTH {
        *counts.entry(nums[i]).or_insert(0) += 1;
    }
    for i in PREAMBLE_LENGTH..nums.len() {
        if !counts.keys().any(|&k| {
            k < nums[i]
                && counts.contains_key(&(nums[i] - k))
                && (k != nums[i] - k || *counts.get(&k).unwrap() > 1)
        }) {
            return Some(nums[i]);
        }
        *counts.entry(nums[i]).or_insert(0) += 1;
        let first_count = counts.get_mut(&nums[i - PREAMBLE_LENGTH]).unwrap();
        if *first_count > 1 {
            *first_count -= 1;
        } else {
            counts.remove(&nums[i - PREAMBLE_LENGTH]);
        }
    }
    None
}

// A contiguous subset of the numbers will add up to the invalid number. The encryption weakness
// is the sum of the largest and smallest numbers from this subset.
fn find_encryption_weakness(invalid: u64, nums: &Vec<u64>) -> Option<u64> {
    let mut sum = 0;
    let mut queue = VecDeque::new();
    for i in 0..nums.len() {
        queue.push_front(nums[i]);
        sum += nums[i];
        while sum > invalid {
            sum -= queue.pop_back().unwrap();
        }
        if sum == invalid && queue.len() > 1 {
            let mut queue = queue.into_iter().collect::<Vec<u64>>();
            queue.sort_unstable();
            return Some(*queue.first().unwrap() + *queue.last().unwrap());
        }
    }
    None
}
