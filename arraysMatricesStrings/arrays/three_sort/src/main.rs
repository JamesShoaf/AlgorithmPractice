use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        let mut keys: Vec<i32> = Vec::new();
        for num in nums {
            if map.contains_key(&num) {
                let val = map.get_mut(&num).unwrap();
                *val += 1;
            } else {
                map.insert(num, 1);
                keys.push(num);
            }
        }
        keys.sort();
        let length = keys.len();
        let mut output: Vec<Vec<i32>> = Vec::new();
        for i in 0..length {
            let i_val = keys[i];
            let j_max = -(i_val / 2);
            if i_val >= 0 { break; }
            for j in i..length {
                let j_val = keys[j];
                if j_val > j_max { break; }
                if (i == j && *map.get(&i_val).unwrap() > 1) 
                || (i != j && *map.get(&j_val).unwrap() > 0) {
                    let k_val = -(i_val + j_val);
                    if (k_val == j_max && *map.get(&j_val).unwrap() > 1)
                    || (k_val > j_max && map.contains_key(&k_val)) {
                        output.push(vec![i_val, j_val, k_val]);
                    }
                }
            }
        }
        if map.contains_key(&0) && *map.get(&0).unwrap() > 2 { output.push(vec![0, 0, 0]); }
        output
    }
}

fn main() {
    let nums: Vec<i32> = vec![0, 0, -1, 1, 2, -1, 0];
    let sums = Solution::three_sum(nums);
    for sum in sums {
        println!("---------");
        for num in sum {
            println!("{}", num);
        }
    }
}
