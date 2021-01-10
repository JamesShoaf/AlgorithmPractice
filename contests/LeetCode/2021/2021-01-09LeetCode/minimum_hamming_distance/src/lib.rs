/*
You are given two integer arrays, source and target, both of length n. You are also given an array
allowedSwaps where each allowedSwaps[i] = [ai, bi] indicates that you are allowed to swap the
elements at index ai and index bi (0-indexed) of array source. Note that you can swap elements at
a specific pair of indices multiple times and in any order.

The Hamming distance of two arrays of the same length, source and target, is the number of
positions where the elements are different. Formally, it is the number of indices i for
0 <= i <= n-1 where source[i] != target[i] (0-indexed).

Return the minimum Hamming distance of source and target after performing any amount of swap
operations on array source.
*/

// union find to find connected components
// generate a map of indices to roots
// for each connected component, compare the count of elements in target and source at those indices
// sum the difference in these components

fn find(i: usize, unions: &mut Vec<usize>) -> usize {
    if i != unions[i] {
        unions[i] = find(unions[i], unions);
    }
    unions[i]
}

fn union(i: usize, j: usize, unions: &mut Vec<usize>) {
    let j_root = find(j, unions);
    unions[j_root] = find(i, unions);
    find(j, unions);
}

use std::collections::HashMap;

pub fn minimum_hamming_distance(
    source: Vec<i32>,
    target: Vec<i32>,
    allowed_swaps: Vec<Vec<i32>>,
) -> i32 {
    let mut unions: Vec<usize> = (0..source.len()).collect();
    for swap in allowed_swaps {
        union(swap[0] as usize, swap[1] as usize, &mut unions);
    }
    println!("unions: {:?}", unions);
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..source.len() {
        find(i, &mut unions);
    }
    for i in 0..source.len() {
        map.entry(unions[i]).or_insert(Vec::new()).push(i);
    }
    println!("map: {:?}", map);
    let mut res = 0;
    for (_, indices) in map.into_iter() {
        let mut counter: HashMap<i32, isize> = HashMap::new();
        for i in indices {
            *counter.entry(target[i]).or_insert(0) += 1;
            *counter.entry(source[i]).or_insert(0) -= 1;
        }
        println!("counter: {:?}", counter);
        res += counter
            .into_iter()
            .filter(|&(_, count)| count > 0)
            .map(|(_, count)| count)
            .sum::<isize>()
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                vec![1, 2, 3, 4],
                vec![2, 1, 4, 5],
                vec![vec![0, 1], vec![2, 3]],
                1,
            ),
            (
                vec![5, 1, 2, 4, 3],
                vec![1, 5, 4, 2, 3],
                vec![vec![0, 4], vec![4, 2], vec![1, 3], vec![1, 4]],
                0,
            ),
        ];
        for (source, target, allowed_swaps, expected) in test_tuples {
            assert_eq!(
                minimum_hamming_distance(source, target, allowed_swaps),
                expected
            );
        }
    }
}
