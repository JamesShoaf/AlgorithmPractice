/*
A good meal is a meal that contains exactly two different food items with a sum of deliciousness
equal to a power of two.

You can pick any two different foods to make a good meal.

Given an array of integers deliciousness where deliciousness[i] is the deliciousness of the i​​​​​​th​​​​​​​​
item of food, return the number of different good meals you can make from this list modulo 109 + 7.

Note that items with different indices are considered different even if they have the same
deliciousness value.
*/

use std::collections::HashMap;

pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    let modulo = (10_i32.pow(9) + 7) as usize;
    let mut res: usize = 0;
    let max_power = (*deliciousness.iter().max().unwrap_or(&1) as f64)
        .log2()
        .ceil() as u32
        + 2;
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for dish in deliciousness {
        *counts.entry(dish).or_insert(0) += 1;
    }
    let keys: Vec<i32> = counts.keys().map(|&val| val).collect();
    for key in keys {
        let key_count = *counts.get(&key).unwrap();
        for power in 0..max_power {
            let complement = 2_i32.pow(power) - key;
            if let Some(&comp_count) = counts.get(&complement) {
                if key == complement {
                    res += (key_count * (key_count - 1) / 2) % modulo;
                } else {
                    res += (key_count * comp_count) % modulo;
                }
                println!(
                    "key: {}, key_count: {}, complement: {}, comp_count: {} res: {}",
                    key, key_count, complement, comp_count, res
                );
            }
            res %= modulo;
        }
        counts.remove(&key);
    }
    res as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count_pairs() {
        let test_tuples = vec![(vec![1, 3, 5, 7, 9], 4), (vec![1, 1, 1, 3, 3, 3, 7], 15)];
        for (nums, expected) in test_tuples {
            assert_eq!(super::count_pairs(nums), expected);
        }
    }
}
