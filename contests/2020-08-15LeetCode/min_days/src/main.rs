/* 
There are n oranges in the kitchen and you decided to eat some of these oranges every day as follows:

    Eat one orange.
    If the number of remaining oranges (n) is divisible by 2 then you can eat  n/2 oranges.
    If the number of remaining oranges (n) is divisible by 3 then you can eat  2*(n/3) oranges.

You can only choose one of the actions per day.

Return the minimum number of days to eat n oranges.
*/

use std::collections::HashMap;
use std::cmp::min;

fn recursive(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if let Some(prev) = memo.get(&n) { return *prev; }
    let min_days = 1 + min(n % 2 + recursive(n / 2, memo), n % 3 + recursive(n / 3, memo));
    memo.insert(n, min_days);
    min_days
}

pub fn min_days(n: i32) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();
    memo.insert(0, 0);
    memo.insert(1, 1);
    recursive(n, &mut memo)
}

fn main() {
    println!("Hello, world!");
}
