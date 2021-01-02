mod lib;
use lib::{part_1, part_2};

fn main() {
    let lines: Vec<Vec<&str>> = include_str!("../input.txt")
        .lines()
        .map(|string| string.split(" = ").collect())
        .collect();
    println!("sum 1: {}", part_1::sum_values(&lines));
    println!("sum 2: {}", part_2::sum_values(&lines));
}
