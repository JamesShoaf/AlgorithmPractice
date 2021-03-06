/*
You are given an undirected graph represented by an integer n, which is the number of nodes, and edges,
where edges[i] = [ui, vi] which indicates that there is an undirected edge between ui and vi. You are
also given an integer array queries.

The answer to the jth query is the number of pairs of nodes (a, b) that satisfy the following conditions:

    a < b
    cnt is strictly greater than queries[j], where cnt is the number of edges incident to a or b.

Return an array answers such that answers.length == queries.length and answers[j] is the answer of the
jth query.

Note that there can be repeated edges.
*/

use std::cmp;
use std::collections::HashMap;

fn evaluate_query(
    query: i32,
    sorted_node_degree: &Vec<usize>,
    node_degree: &Vec<usize>,
    edge_degree: &HashMap<(i32, i32), usize>,
) -> i32 {
    let query = query as usize;
    let n = node_degree.len();
    // count all node pairs such that deg[a] + deg[b] > query
    let mut count = 0;
    // n  - tail is the number of nodes for the current head node such that
    // degree[head] + degree[tail] > query
    let mut tail = n;
    // since a != b for node pairs (a, b) head iterates from 0..n - 1 and tail from n-1..=n
    for head in 0..n - 1 {
        // if deg[tail] + deg[head] > query, deg[tail + 1] + deg[head + 1] > query
        tail = cmp::max(tail, head + 1);
        // decrease tail (thus increasing n - tail) until deg[tail] + deg[head] <= query (or tail -1 == head)
        while tail - 1 > head && sorted_node_degree[head] + sorted_node_degree[tail - 1] > query {
            tail -= 1;
        }
        // finally, add the number of valid node pairs for the current head
        count += n - tail;
    }
    // remove all pairs where deg[a] + deg[b] - edge_deg[(a, b)] <= query
    count -= edge_degree
        .iter()
        .map(|(&(a, b), c)| ((a as usize - 1, b as usize - 1), c))
        .filter(|&((a, b), c)| node_degree[a] + node_degree[b] - c <= query)
        .filter(|&((a, b), _)| node_degree[a] + node_degree[b] > query)
        .count();
    count as i32
}

pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    // count degree for each node and degree of each edge
    let mut node_degree: Vec<usize> = vec![0; n as usize];
    let mut edge_degree: HashMap<(i32, i32), usize> = HashMap::new();
    for edge in edges {
        let (a, b) = (edge[0], edge[1]);
        node_degree[a as usize - 1] += 1;
        node_degree[b as usize - 1] += 1;
        *edge_degree
            .entry((cmp::min(a, b), cmp::max(a, b)))
            .or_insert(0) += 1;
    }
    // sort the degree of each edge in ascending order
    let mut sorted_node_degree = node_degree.clone();
    sorted_node_degree.sort_unstable();
    // evaluate each query
    queries
        .into_iter()
        .map(|query| evaluate_query(query, &sorted_node_degree, &node_degree, &edge_degree))
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
