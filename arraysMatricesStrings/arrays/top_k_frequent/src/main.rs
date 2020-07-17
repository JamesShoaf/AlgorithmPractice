use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solution {}

impl Solution {
    pub fn top_k_frequent_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() as i32 == k { return nums; }
        let mut map: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            if let Some(val) = map.insert(num, 1) {
                map.insert(num, val + 1);
            }
        }
        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        for (val, count) in &map {
            heap.push((*count, *val));
        }
        let mut output: Vec<i32> = Vec::new();
        for _ in 0..k {
            let (_, val) = heap.pop().unwrap();
            output.push(val);
        }
        output
    }

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let length = nums.len() as i32;
        if length == k { return nums; }
        let mut map: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            if let Some(val) = map.insert(num, 1) {
                map.insert(num, val + 1);
            }
        }
        
        let mut map = map.into_iter().collect::<Vec<(i32, i32)>>();
        map.sort_by_key(|&(a, b)| (Reverse(b), a));
        map[0..k as usize].into_iter().map(|&(a, _)| a).collect()
    }
}

fn main() {
    let nums = vec![3, 5, 2, 2, 8, 20, 1, 2, 1, 2, 1];
    println!("{:#?}", Solution::top_k_frequent(nums, 2));
}
