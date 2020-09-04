use lib::partition_labels;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 2 { panic!("Invalid number of arguments: Expected 1, but received {}", args.len() - 1); }
    println!("{:?}", partition_labels::partition_labels(args.pop().unwrap()));
}