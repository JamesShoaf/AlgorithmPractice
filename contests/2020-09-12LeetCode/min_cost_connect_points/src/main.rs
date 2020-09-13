use lib::min_cost_connect_points;

fn main() {
    let test = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
    let output = min_cost_connect_points(test);
    println!("{}", output);
}