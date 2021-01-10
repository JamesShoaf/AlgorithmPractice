/*
You are given an integer array jobs, where jobs[i] is the amount of time it takes to complete the
ith job.

There are k workers that you can assign jobs to. Each job should be assigned to exactly one worker.
The working time of a worker is the sum of the time it takes to complete all jobs assigned to them.
Your goal is to devise an optimal assignment such that the maximum working time of any worker is
minimized.

Return the minimum possible maximum working time of any assignment.
*/

use std::cmp;
use std::collections::HashSet;

fn helper(i: usize, jobs: &Vec<i32>, workers: &mut Vec<i32>, res: &mut i32) {
    if i == jobs.len() {
        *res = cmp::min(*res, *workers.iter().max().unwrap());
        return;
    }
    let mut visited: HashSet<i32> = HashSet::new();
    for j in 0..workers.len() {
        if !visited.contains(&workers[j]) {
            visited.insert(workers[j]);
            if workers[j] + jobs[i] < *res {
                workers[j] += jobs[i];
                helper(i + 1, jobs, workers, res);
                workers[j] -= jobs[i];
            }
        }
    }
}

pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
    let mut res = std::i32::MAX;
    let mut workers = vec![0; k as usize];
    helper(0, &jobs, &mut workers, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![(vec![38, 49, 91, 59, 14, 76, 84], 3, 140)];
        for (jobs, k, expected) in test_tuples {
            assert_eq!(minimum_time_required(jobs, k), expected);
        }
    }
}
