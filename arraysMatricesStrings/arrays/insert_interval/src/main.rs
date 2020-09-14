use lib::insert;

fn main() {
    let intervals = vec![vec![1, 5]];
    let new_interval = vec![0, 3];
    let intervals = insert(intervals, new_interval);
    println!("{:?}", intervals);
}