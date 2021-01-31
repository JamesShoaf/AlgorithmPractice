/*
There is an integer array nums that consists of n unique elements, but you have forgotten it.
However, you do remember every pair of adjacent elements in nums.

You are given a 2D integer array adjacentPairs of size n - 1 where each
adjacentPairs[i] = [ui, vi] indicates that the elements ui and vi are adjacent in nums.

It is guaranteed that every adjacent pair of elements nums[i] and nums[i+1] will exist in
adjacentPairs, either as [nums[i], nums[i+1]] or [nums[i+1], nums[i]]. The pairs can appear
in any order.

Return the original array nums. If there are multiple solutions, return any of them.
*/

use std::collections::HashMap;

pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for pair in adjacent_pairs {
        map.entry(pair[0]).or_insert(Vec::new()).push(pair[1]);
        map.entry(pair[1]).or_insert(Vec::new()).push(pair[0]);
    }
    let mut current = 0;
    for (&k, v) in map.iter() {
        if v.len() == 1 {
            current = k;
            break;
        }
    }
    let mut res = vec![current];
    while map.contains_key(&current) {
        let next = map.get(&current).unwrap().clone();
        map.remove(&current);
        for num in next {
            if map.contains_key(&num) {
                current = num;
                res.push(current);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
