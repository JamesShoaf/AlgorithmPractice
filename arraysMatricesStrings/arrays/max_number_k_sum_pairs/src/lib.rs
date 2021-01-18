use std::cmp;
use std::collections::HashMap;

pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        *counter.entry(num).or_insert(0) += 1;
    }
    let vals: Vec<i32> = counter.keys().map(|&k| k).collect();
    for val in vals {
        let complement = k - val;
        if let Some(&count1) = counter.get(&val) {
            if val == complement {
                res += count1 / 2;
            } else if let Some(&count2) = counter.get(&complement) {
                res += cmp::min(count1, count2);
            }
        }
        counter.remove(&val);
        counter.remove(&complement);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![(vec![1, 2, 3, 4], 5, 2), (vec![3, 1, 3, 4, 3], 6, 1)];
        for (nums, k, expected) in test_tuples {
            assert_eq!(max_operations(nums, k), expected);
        }
    }
}
