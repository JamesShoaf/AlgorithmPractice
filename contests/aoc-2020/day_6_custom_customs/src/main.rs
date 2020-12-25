use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    if let Ok(f) = File::open("input.txt") {
        let f = BufReader::new(f);
        let mut customs_groups: Vec<Vec<String>> = Vec::new();
        let mut group: Vec<String> = Vec::new();
        for line in f.lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    customs_groups.push(group);
                    group = Vec::new();
                } else {
                    group.push(line);
                }
            }
        }
        customs_groups.push(group);
        println!("{:?}", customs_groups.last().unwrap());
        let or_total: usize = customs_groups
            .iter()
            .map(|group| collect_or_group(group).len())
            .sum();
        println!("Sum of or_group counts: {}", or_total);
        let and_total: usize = customs_groups
            .iter()
            .map(|group| collect_and_group(group).len())
            .sum();
        println!("Sum of and_group counts: {}", and_total);
    }
}

use std::collections::HashSet;

// collect list of all letters any member of the group contains
fn collect_or_group(group: &Vec<String>) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    for form in group {
        for c in form.chars() {
            set.insert(c);
        }
    }
    set
}

// collect list of all letters all members of the group contain
fn collect_and_group(group: &Vec<String>) -> HashSet<char> {
    let mut res = HashSet::new();
    if group.is_empty() {
        return res;
    }
    for c in group[0].chars() {
        res.insert(c);
    }
    for i in 1..group.len() {
        let mut temp_set: HashSet<char> = HashSet::new();
        for c in group[i].chars() {
            temp_set.insert(c);
        }
        res = res.intersection(&temp_set).map(|&c| c).collect();
    }
    res
}
