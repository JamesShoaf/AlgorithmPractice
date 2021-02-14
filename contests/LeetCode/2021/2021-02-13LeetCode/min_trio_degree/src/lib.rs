/*
You are given an undirected graph. You are given an integer n which is the number of nodes
in the graph and an array edges, where each edges[i] = [ui, vi] indicates that there is an
undirected edge between ui and vi.

A connected trio is a set of three nodes where there is an edge between every pair of them.

The degree of a connected trio is the number of edges where one endpoint is in the trio,
and the other is not.

Return the minimum degree of a connected trio in the graph, or -1 if the graph has no
connected trios.
*/

use std::cmp;
use std::collections::HashSet;

pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n + 1];
    for edge in edges {
        let (a, b) = (edge[0] as usize, edge[1] as usize);
        graph[a].insert(b);
        graph[b].insert(a);
    }
    let mut visited: HashSet<usize> = HashSet::new();
    let mut res: Option<usize> = None;
    for i in 1..n + 1 {
        visited.insert(i);
        for &j in &graph[i] {
            if !visited.contains(&j) {
                for &k in graph[i].intersection(&graph[j]) {
                    let degree = graph[i].len() + graph[j].len() + graph[k].len() - 6;
                    if degree == 0 {
                        return 0;
                    }
                    if let Some(res) = res.as_mut() {
                        *res = cmp::min(*res, degree);
                    } else {
                        res = Some(degree);
                    }
                }
            }
        }
    }
    if let Some(res) = res {
        return res as i32;
    }
    -1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
