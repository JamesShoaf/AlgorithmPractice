use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    if let Ok(f) = File::open("input.txt") {
        let f = BufReader::new(f);
        let joltages: Vec<u64> = f
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|string| string.parse().ok())
            .collect();
        if let Some(diffs) = get_joltage_diffs(&joltages) {
            println!(
                "1 Jolt: {}, 2 Jolt: {}, 3 Jolt: {}",
                diffs[0], diffs[1], diffs[2]
            );
            println!("product: {}", diffs[0] * diffs[2]);
        }
        println!(
            "possible combinations: {}",
            possible_combinations(&joltages)
        );
    }
}

fn get_joltage_diffs(joltages: &Vec<u64>) -> Option<[usize; 3]> {
    let mut joltages = joltages.clone();
    joltages.push(0);
    joltages.sort_unstable();
    joltages.push(*joltages.last().unwrap() + 3);
    let mut res = [0; 3];
    for i in 1..joltages.len() {
        let diff = joltages[i] - joltages[i - 1];
        Some(()).filter(|_| diff > 0 && diff <= 3)?;
        res[(diff - 1) as usize] += 1;
    }
    Some(res)
}

fn possible_combinations(joltages: &Vec<u64>) -> usize {
    let mut joltages = joltages.clone();
    joltages.push(0);
    joltages.sort_unstable();
    let mut dp = vec![0; joltages.len()];
    *dp.last_mut().unwrap() = 1;
    for i in (0..joltages.len() - 1).rev() {
        for j in 1..4 {
            if i + j == joltages.len() || joltages[i + j] - joltages[i] > 3 {
                break;
            }
            dp[i] += dp[i + j];
        }
    }
    *dp.first().unwrap()
}

mod tests {
    #[test]
    fn test_possible_combinations() {
        let test_tuples = vec![
            (Vec::new(), 1),
            (vec![1], 1),
            (vec![1, 2], 2),
            (vec![1, 2, 3], 4),
            (vec![1, 2, 3, 4], 7),
            (vec![1, 2, 3, 4, 5], 13),
            (vec![1, 3, 6], 2),
        ];
        for (joltages, expected) in test_tuples {
            assert_eq!(super::possible_combinations(&joltages), expected);
        }
    }
}
