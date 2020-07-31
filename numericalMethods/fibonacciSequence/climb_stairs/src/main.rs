/* 
You are climbing a stair case. It takes n steps to reach to the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
*/

struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut prev_steps: Vec<i32> = vec![1, 1];
        for i in 2..=n {
            prev_steps[(i % 2) as usize] += prev_steps[((i + 1) % 2) as usize];
        }
        prev_steps[(n % 2) as usize]
    }

    pub fn climb_k_stairs(n: i32, k: usize) -> i32 {
        let mut prev_steps: Vec<i32> = vec![0; k];
        prev_steps[0] = 1;
        for i in 1..=(n as usize) {
            prev_steps[i % k] = prev_steps.iter().fold(0, |acc, num| acc + num);
        }
        prev_steps[n as usize % k]
    }
}

fn main() {
    for j in 1..5 {
        for i in 0..10 {
            let possible_ways = Solution::climb_k_stairs(i, j);
            println!("{} ways to climb {} steps up to {} at a time", possible_ways, i, j);
        }
    }
}
