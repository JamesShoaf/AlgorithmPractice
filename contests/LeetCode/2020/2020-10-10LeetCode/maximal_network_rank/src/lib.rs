/* 
There is an infrastructure of n cities with some number of roads connecting these cities.
Each roads[i] = [ai, bi] indicates that there is a bidirectional road between cities ai and bi.

The network rank of two different cities is defined as the total number of directly connected
roads to either city. If a road is directly connected to both cities, it is only counted once.

The maximal network rank of the infrastructure is the maximum network rank of all pairs of
different cities.

Given the integer n and the array roads, return the maximal network rank of the entire infrastructure.
*/

use std::collections::HashSet;

pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let mut nodes = vec![HashSet::new(); n as usize];
    for road in roads {
        nodes[road[0] as usize].insert(road[1]);
        nodes[road[1] as usize].insert(road[0]);
    }
    let mut max = 0;
    for i in 0..n {
        for j in i + 1..n {
            let roads = nodes[i as usize].len() + nodes[j as usize].len()
                - if nodes[i as usize].contains(&j) { 1 } else { 0 };
            if roads > max {
                max = roads;
            }
        }
    }
    max as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
