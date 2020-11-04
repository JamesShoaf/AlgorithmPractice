/* 
A tree is an undirected graph in which any two vertices are connected by exactly one path. In other
words, any connected graph without simple cycles is a tree.

Given a tree of n nodes labelled from 0 to n - 1, and an array of n - 1 edges where
edges[i] = [ai, bi] indicates that there is an undirected edge between the two nodes ai and bi in
the tree, you can choose any node of the tree as the root. When you select a node x as the root,
the result tree has height h. Among all possible rooted trees, those with minimum height
(i.e. min(h))  are called minimum height trees (MHTs).

Return a list of all MHTs' root labels. You can return the answer in any order.

The height of a rooted tree is the number of edges on the longest downward path between the root
and a leaf.
*/

// this is equivalent to finding the center or centers of the tree

use std::collections::HashSet;

#[derive(Clone)]
struct GraphNode {
    degree: usize,
    neighbors: HashSet<usize>,
}

fn convert_edges_to_graph(n: i32, edges: Vec<Vec<i32>>) -> Vec<GraphNode> {
    let mut res = vec![GraphNode {degree: 0, neighbors: HashSet::new()}; n as usize];
    for edge in edges {
        let (from, to) = (edge[0] as usize, edge[1] as usize);
        res[from].degree += 1;
        res[from].neighbors.insert(to);
        res[to].degree += 1;
        res[to].neighbors.insert(from);
    }
    res
}

pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    if edges.is_empty() { return vec![0]; }
    let mut graph = convert_edges_to_graph(n, edges);
    let mut leaf_count = 0;
    loop {
        let leaves: Vec<usize> = graph.iter()
            .enumerate()
            .filter(|(_, node)| node.degree < 2)
            .map(|(i, _)| i).collect();
        leaf_count += leaves.len();
        if leaf_count == n as usize {
            return leaves.into_iter().map(|i| i as i32).collect()
        }
        for leaf in leaves {
            let neighbor: usize = *graph[leaf].neighbors.iter().collect::<Vec<&usize>>()[0];
            // set leaf's degree to a dummy value so it will not be added to leaves again
            graph[leaf].degree = 2;
            graph[neighbor].degree -= 1;
            graph[neighbor].neighbors.remove(&leaf);
        }
    }
}

/* 
#1 solution for comparison

use std::collections::VecDeque;

pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    if n == 1 {
        return vec![0];
    }
    let mut degrees = vec![0; n as usize];
    let mut queue = VecDeque::new();
    let mut graph = vec![Vec::new(); n as usize];
    let mut num_vertexes = n;

    edges.iter().for_each(|edge| {
        graph[edge[0] as usize].push(edge[1]);
        graph[edge[1] as usize].push(edge[0]);
    });

    for edge in edges {
        let first_vertex = edge[0] as usize;
        let second_vertex = edge[1] as usize;
        degrees[first_vertex] = degrees[first_vertex] + 1;
        degrees[second_vertex] = degrees[second_vertex] + 1;
    }

    degrees.iter().enumerate().for_each(|(idx, degree)| if *degree == 1 {queue.push_back(idx)});

    while num_vertexes > 2 {
        for _ in 0..queue.len() {
            let leaf_to_remove = queue.pop_front().unwrap() as usize;
            num_vertexes -= 1;

            degrees[leaf_to_remove] = degrees[leaf_to_remove] - 1;

            for adjacent_vertex in &graph[leaf_to_remove] {
                degrees[*adjacent_vertex as usize] = degrees[*adjacent_vertex as usize] - 1;

                if degrees[*adjacent_vertex as usize] == 1 {
                    queue.push_back(*adjacent_vertex as usize);
                }
            }
        }
    }

    return queue.iter().map(|val| *val as i32).collect();
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (6,
            vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]],
            vec![3, 4]),
            (1,
            Vec::new(),
            vec![0]),
            (2,
            vec![vec![0, 1]],
            vec![0, 1]),
        ];
        for (n, edges, expected) in test_tuples {
            assert_eq!(find_min_height_trees(n, edges), expected);
        }
    }
}
