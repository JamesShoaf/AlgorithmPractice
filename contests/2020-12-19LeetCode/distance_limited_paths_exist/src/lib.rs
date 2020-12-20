/*
An undirected graph of n nodes is defined by edgeList, where edgeList[i] = [ui, vi, disi] denotes
an edge between nodes ui and vi with distance disi. Note that there may be multiple edges between
two nodes.

Given an array queries, where queries[j] = [pj, qj, limitj], your task is to determine for each
queries[j] whether there is a path between pj and qj such that each edge on the path has a
distance strictly less than limitj .

Return a boolean array answer, where answer.length == queries.length and the jth value of answer is
true if there is a path for queries[j] is true, and false otherwise.
*/

// Union Find implementation - Sort queries and edges by distance in ascending order, then union
// nodes from edges shorter than the query distance. Then, evaluate the query.
// O(Nlog(N)) time, O(N) space
fn union(i: usize, j: usize, nodes: &mut Vec<usize>) {
    let i_root = find(i, nodes);
    let j_root = find(j, nodes);
    nodes[j_root] = i_root;
    find(j, nodes);
}

fn find(i: usize, nodes: &mut Vec<usize>) -> usize {
    if nodes[i] != i {
        nodes[i] = find(nodes[i], nodes);
    }
    nodes[i]
}

pub fn distance_limited_paths_exist(
    n: i32,
    mut edge_list: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    let mut res = vec![false; queries.len()];
    let mut nodes: Vec<usize> = Vec::new();
    for i in 0..n as usize {
        nodes.push(i);
    }
    // dist, a, b, i
    let mut queries: Vec<(i32, usize, usize, usize)> = queries
        .into_iter()
        .enumerate()
        .map(|(i, query)| (query[2], query[0] as usize, query[1] as usize, i))
        .collect();
    queries.sort();
    edge_list.sort_unstable_by_key(|edge| edge[2]);
    let mut i = 0;
    for (dist, a, b, j) in queries {
        while i < edge_list.len() && edge_list[i][2] < dist {
            let (c, d) = (edge_list[i][0] as usize, edge_list[i][1] as usize);
            union(c, d, &mut nodes);
            i += 1;
        }
        if find(a, &mut nodes) == find(b, &mut nodes) {
            res[j] = true;
        }
    }
    res
}

// initial implementation - Builds graph in order of ascending edge length and strongly connects
// all connected edges - O(E*V^2) time in worst case with significant amortization improvements
// use std::cmp;
// use std::collections::{HashMap, HashSet, VecDeque};

// fn bfs_update(a: i32, b: i32, dist: i32, nodes: &mut HashMap<i32, HashMap<i32, i32>>) {
//     let mut queue: VecDeque<i32> = VecDeque::from(vec![a, b]);
//     let mut connected: HashSet<i32> = HashSet::new();
//     while let Some(node) = queue.pop_front() {
//         connected.insert(node);
//         for &k in nodes
//             .get(&node)
//             .unwrap()
//             .keys()
//             .filter(|&k| !connected.contains(k))
//         {
//             queue.push_back(k);
//         }
//     }
//     for &node in connected.iter() {
//         for &neighbor in connected.iter() {
//             let prev = nodes
//                 .get_mut(&node)
//                 .unwrap()
//                 .entry(neighbor)
//                 .or_insert(dist);
//             *prev = cmp::min(*prev, dist);
//         }
//     }
// }

// pub fn distance_limited_paths_exist(
//     n: i32,
//     mut edge_list: Vec<Vec<i32>>,
//     queries: Vec<Vec<i32>>,
// ) -> Vec<bool> {
//     edge_list.sort_unstable_by_key(|edge| edge[2]);
//     let mut nodes: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
//     for i in 0..n {
//         nodes
//             .entry(i)
//             .or_insert(HashMap::new())
//             .entry(i)
//             .or_insert(0);
//     }
//     for edge in edge_list {
//         let (a, b, dist) = (edge[0], edge[1], edge[2]);
//         if !nodes.get(&a).unwrap().contains_key(&b) {
//             bfs_update(a, b, dist, &mut nodes);
//         }
//     }
//     queries.into_iter().map(|edge| {
//         if let Some(&connection) = nodes.get(&edge[0]).unwrap().get(&edge[1]) {
//             if connection < edge[2] {
//                 return true;
//             }
//         }
//         false
//     }).collect()
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
