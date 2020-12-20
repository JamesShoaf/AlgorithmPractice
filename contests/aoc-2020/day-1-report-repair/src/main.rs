use std::fs;

fn main() {
    let reports: Vec<i32> = fs::read_to_string("input.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("day 1 part 1: {}", repair_report(reports.clone()));
    println!("day 1 part 2: {}", repair_report_ii(reports));
}

use std::collections::HashSet;
const TARGET: i32 = 2020;

fn repair_report(reports: Vec<i32>) -> i32 {
    let mut set: HashSet<i32> = HashSet::new();
    for num in reports {
        if set.contains(&(TARGET - num)) {
            return num * (TARGET - num);
        }
        set.insert(num);
    }
    -1
}

fn repair_report_ii(reports: Vec<i32>) -> i32 {
    for i in 0..reports.len() {
        for j in i + 1..reports.len() {
            for k in j + 1..reports.len() {
                if reports[i] + reports[j] + reports[k] == TARGET {
                    return reports[i] * reports[j] * reports[k];
                }
            }
        }
    }
    -1
}

#[test]
fn test_repair() {
    let reports = vec![1721, 979, 366, 299, 675, 1456];
    assert!(repair_report(reports) == 514579);
}
#[test]
fn test_repair_ii() {
    let reports = vec![1721, 979, 366, 299, 675, 1456];
    assert!(repair_report_ii(reports) == 241861950);
}
