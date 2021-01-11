/*
Given an integer array instructions, you are asked to create a sorted array from the elements in
instructions. You start with an empty container nums. For each element from left to right in
instructions, insert it into nums. The cost of each insertion is the minimum of the following:

    The number of elements currently in nums that are strictly less than instructions[i].
    The number of elements currently in nums that are strictly greater than instructions[i].

For example, if inserting element 3 into nums = [1,2,3,5], the cost of insertion is min(2, 1)
(elements 1 and 2 are less than 3, element 5 is greater than 3) and nums will become [1,2,3,3,5].

Return the total cost to insert all elements from instructions into nums. Since the answer may be
large, return it modulo 109 + 7
*/

use fenwick::array::{prefix_sum, update};
use std::cmp;

pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
    const MODULO: usize = 1_000_000_007;
    let mut arr = [0; 100_001];
    let fw = &mut arr;
    let mut res = 0;
    for i in 0..instructions.len() {
        let j = instructions[i] as usize;
        res += cmp::min(prefix_sum(fw, j - 1), i - prefix_sum(fw, j));
        res %= MODULO;
        update(fw, j, 1);
    }
    res as i32
}

/*
#1 implementation for comparison
fn get(a:&Vec<i32>, idx: i32) -> i32 {
    let mut i = idx + 1;
    let mut sum = 0;
    while i > 0 {
        sum += a[i as usize];
        // subtract least significant bit
        i -= i & (-i);
    }
    sum
}
fn update(a:&mut Vec<i32>, idx: i32) {
    let mut i = idx + 1;
    let mut sum = 0;
    while i <= 100_005 {
        a[i as usize] += 1;
        // add least significant bit
        i += i & (-i);
    }
}
pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
    let mut a = vec![0; 100_100 ];
    let di = 1_000_000_007;
    let mut res = 0;
    for x in instructions {
        res = (res + std::cmp::min(get(&a, 100_005) - get(&a, x), get(&a, x-1))) % di;
        update(&mut a, x)
    }
    res
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_sorted_array() {
        let test_tuples = vec![
            (vec![1, 5, 6, 2], 1),
            (vec![1, 2, 3, 6, 5, 4], 3),
            (vec![1, 3, 3, 3, 2, 4, 2, 1, 2], 4),
        ];
        for (instructions, expected) in test_tuples {
            assert_eq!(create_sorted_array(instructions), expected);
        }
    }
}
