mod solution;
use solution::Solution;

fn main() {
    let vec = vec![0, 1, 2, 3, 5, 6, 9, 10, 11, 15, 21, 10000000];
    for int in vec {
        println!("{} rows can be made with {} coins.", Solution::arrange_coins(int), int);
    }
}