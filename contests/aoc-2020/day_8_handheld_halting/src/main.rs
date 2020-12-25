use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    if let Ok(f) = File::open("input.txt") {
        let f = BufReader::new(f);
        let mut ops: Vec<(String, i32)> = f
            .lines()
            .filter(|line| line.is_ok())
            .map(|line| line.unwrap())
            .map(|line| {
                let mut line = line.split_whitespace();
                let op = line.next().unwrap().to_string();
                let val = line.next().unwrap().parse().unwrap();
                (op, val)
            })
            .collect();
        println!("Last Acc: {}", find_last_acc(&ops));
        println!("Fixed Acc: {}", find_fixing_change(&mut ops));
    }
}

fn find_last_acc(ops: &Vec<(String, i32)>) -> i32 {
    let mut visited = HashSet::new();
    let mut i = 0;
    let mut acc = 0;
    while !visited.contains(&i) {
        visited.insert(i);
        match &ops[i as usize].0[..] {
            "jmp" => i += ops[i as usize].1,
            "acc" => {
                acc += ops[i as usize].1;
                i += 1;
            }
            "nop" => i += 1,
            _ => (),
        }
    }
    acc
}

fn swap_jmp_nop(ops: &mut Vec<(String, i32)>, i: usize) {
    match &ops[i].0[..] {
        "jmp" => ops[i].0 = "nop".to_string(),
        "nop" => ops[i].0 = "jmp".to_string(),
        _ => (),
    }
}

fn find_fixing_change(ops: &mut Vec<(String, i32)>) -> i32 {
    for i in 0..ops.len() {
        match &ops[i].0[..] {
            "jmp" | "nop" => {
                swap_jmp_nop(ops, i);
                if let Some(val) = get_terminating_val(ops) {
                    return val;
                }
                swap_jmp_nop(ops, i);
            }
            _ => (),
        }
    }
    0
}

fn get_terminating_val(ops: &Vec<(String, i32)>) -> Option<i32> {
    let mut visited = HashSet::new();
    let mut i = 0;
    let mut acc = 0;
    while !visited.contains(&i) && ops.len() > i as usize {
        visited.insert(i);
        match &ops[i as usize].0[..] {
            "jmp" => i += ops[i as usize].1,
            "acc" => {
                acc += ops[i as usize].1;
                i += 1;
            }
            "nop" => i += 1,
            _ => (),
        }
    }
    Some(acc).filter(|_| i as usize == ops.len())
}
