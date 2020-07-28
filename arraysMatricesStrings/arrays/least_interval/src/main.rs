use std::collections::{ BinaryHeap, HashMap };

struct Solution {}

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut task_count = tasks.len();
        if n == 0 { return task_count as i32; }
        let mut counter: HashMap<char, i32> = HashMap::new();
        for task in &tasks {
            // start at -1, since one of each task will be queued immediately
            let count = counter.entry(*task).or_insert(-1);
            *count += 1;
        }
        // max heap of (unqueued task count, task name)
        let mut heap: BinaryHeap<(i32, char)> = BinaryHeap::new();
        for (task, count) in &counter { heap.push((*count, *task)); }
        let mut time: i32 = 0;
        loop {
            let mut most_frequent: Vec<(i32, char)> = Vec::new();
            for _ in 0..n + 1 {
                if task_count == 0 { return time; }
                time += 1;
                if let Some((_, task)) = heap.pop() {
                    task_count -= 1;
                    let count = counter.get_mut(&task).unwrap();
                    if *count > 0 {
                        *count -= 1;
                        most_frequent.push((*count, task));
                    }
                }
            }
            for tuple in most_frequent { heap.push(tuple);}
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<char>, i32, i32)> = vec![
        (vec!['A', 'A', 'A', 'B', 'B', 'B'], 2, 8),
        (vec!['A', 'A', 'A', 'B', 'B', 'B'], 0, 6),
        (vec!['A', 'A', 'A',  'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'], 2, 16),
    ];
    for (tasks, n, expected) in test_cases {
        assert_eq!(Solution::least_interval(tasks, n), expected);
    }
}
