/*
There is a tree (i.e., a connected, undirected graph that has no cycles) consisting of n nodes
numbered from 0 to n - 1 and exactly n - 1 edges. Each node has a value associated with it, and
the root of the tree is node 0.

To represent this tree, you are given an integer array nums and a 2D array edges. Each nums[i]
represents the ith node's value, and each edges[j] = [uj, vj] represents an edge between nodes
uj and vj in the tree.

Two values x and y are coprime if gcd(x, y) == 1 where gcd(x, y) is the greatest common divisor
of x and y.

An ancestor of a node i is any other node on the shortest path from node i to the root. A node
is not considered an ancestor of itself.

Return an array ans of size n, where ans[i] is the closest ancestor to node i such that nums[i]
and nums[ans[i]] are coprime, or -1 if there is no such ancestor.
*/

fn gcd(a: i32, b: i32) -> i32 {
    if b > a {
        return gcd(b, a);
    }
    if a == b || b == 0 {
        return a;
    }
    if a & 1 == 0 {
        if b & 1 == 0 {
            return gcd(a >> 1, b >> 1) << 1;
        }
        return gcd(a >> 1, b);
    }
    if b & 1 == 0 {
        return gcd(a, b >> 1);
    }
    gcd(a - b, b)
}

use std::collections::{HashMap, HashSet};

fn dfs(
    node: usize,
    height: u32,
    nums: &Vec<i32>,
    tree: &HashMap<usize, Vec<usize>>,
    path: &mut HashMap<i32, Vec<(u32, usize)>>,
    visited: &mut HashSet<usize>,
    res: &mut Vec<i32>,
) {
    //mark current node as visited
    visited.insert(node);
    // scan the path to the current node for the ancestor with gcd == 1 and highest depth
    if let Some(&(_, ancestor)) = path
        .iter()
        .filter(|(&k, _)| gcd(nums[node], k) == 1)
        .filter_map(|(_, vec)| vec.last())
        .max()
    {
        // add that ancestor to the result, if it exists
        res[node] = ancestor as i32;
    }
    // add the current node to the path
    path.entry(nums[node])
        .or_insert(Vec::new())
        .push((height, node));
    // recurse on children
    for &child in tree.get(&node).unwrap() {
        if !visited.contains(&child) {
            dfs(child, height + 1, nums, tree, path, visited, res);
        }
    }
    // remove the current node from the path
    path.get_mut(&nums[node]).unwrap().pop();
}

pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut tree: HashMap<usize, Vec<usize>> = HashMap::new();
    for edge in edges {
        let (a, b) = (edge[0] as usize, edge[1] as usize);
        tree.entry(a).or_insert(Vec::new()).push(b);
        tree.entry(b).or_insert(Vec::new()).push(a);
    }
    let mut res = vec![-1; nums.len()];
    let mut path: HashMap<i32, Vec<(u32, usize)>> = HashMap::new();
    let mut visited: HashSet<usize> = HashSet::new();
    dfs(0, 0, &nums, &tree, &mut path, &mut visited, &mut res);
    res
}

// fn find_coprime_ancestor(
//     node: usize,
//     ancestor: usize,
//     parents: &Vec<Option<usize>>,
//     nums: &Vec<i32>,
// ) -> Option<usize> {
//     if node != ancestor && gcd(nums[node], nums[ancestor]) == 1 {
//         return Some(ancestor);
//     }
//     if let Some(parent) = parents[ancestor] {
//         return find_coprime_ancestor(node, parent, parents, nums);
//     }
//     None
// }

// pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
//     let mut connected: Vec<Vec<usize>> = vec![Vec::new(); nums.len()];
//     for edge in edges {
//         let a = edge[0] as usize;
//         let b = edge[1] as usize;
//         connected[a].push(b);
//         connected[b].push(a);
//     }
//     let mut parents: Vec<Option<usize>> = vec![None; nums.len()];
//     let mut visited: HashSet<usize> = HashSet::new();
//     let mut queue: VecDeque<usize> = VecDeque::new();
//     queue.push_back(0);
//     while let Some(parent) = queue.pop_front() {
//         visited.insert(parent);
//         for &child in connected[parent]
//             .iter()
//             .filter(|child| !visited.contains(&child))
//         {
//             parents[child] = Some(parent);
//             queue.push_back(child);
//         }
//     }
//     (0..nums.len())
//         .map(|i| {
//             if let Some(ancestor) = find_coprime_ancestor(i, i, &parents, &nums) {
//                 return ancestor as i32;
//             } else {
//                 return -1;
//             }
//         })
//         .collect()
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
