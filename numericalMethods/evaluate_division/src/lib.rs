/* 
You are given equations in the format A / B = k, where A and B are variables represented as
strings, and k is a real number (floating-point number). Given some queries, return the answers. If
the answer does not exist, return -1.0.

The input is always valid. You may assume that evaluating the queries will result in no division by
zero and there is no contradiction.
*/

// use union find to group nodes into connected components
// then compress the paths from leaf nodes to the root node
// then calculate results for queries of nodes in connected components from the 

use std::collections::HashMap;

// convert node name to index, or insert if the node is not in the graph
fn get_node_index(key: String, map: &mut HashMap<String, usize>, nodes: &mut Vec<usize>, ratios: &mut Vec<f64>) -> usize {
    if let Some(index) = map.get(&key) {
        return *index;
    }
    map.insert(key, nodes.len());
    nodes.push(nodes.len());
    ratios.push(1.0_f64);
    nodes.len() - 1
}

// 
fn find(index: usize, nodes: &mut Vec<usize>, ratios: &mut Vec<f64>) -> usize {
    if nodes[index] == index {
        return index;
    }
    compress(index, nodes, ratios);
    nodes[index]
}

// recursively compress the path from the leaf to the root
fn compress(index: usize, nodes: &mut Vec<usize>, ratios: &mut Vec<f64>) {
    if nodes[index] != index {
        compress(nodes[index], nodes, ratios);
        ratios[index] *= ratios[nodes[index]];
        nodes[index] = nodes[nodes[index]];
    }
}

// if a and b are not already connected, connect them
fn union(a: usize, b: usize, ratio: f64, nodes: &mut Vec<usize>, ratios: &mut Vec<f64>) {
    let a_root = find(a, nodes, ratios);
    let b_root = find(b, nodes, ratios);
    if a_root != b_root {
        nodes[b_root] = a_root;
        // multiply/divide ratios to get new ratio from b_root to a
        ratios[b_root] *= ratios[a] / (ratio * ratios[b]);
    }
}

// Convert Vec<String> to its component Strings and unite components
fn insert_eq(equation: Vec<String>, value: f64, map: &mut HashMap<String, usize>, nodes: &mut Vec<usize>, ratios: &mut Vec<f64>) {
    if equation.len() < 2 {
        panic!("less than two variables found in equation {:?}", equation);
    }
    let mut equation = equation.into_iter();
    let a_index = get_node_index(equation.next().unwrap(), map, nodes, ratios);
    let b_index = get_node_index(equation.next().unwrap(), map, nodes, ratios);
    union(a_index, b_index, value, nodes, ratios);
}

// Convert Vec<String> to its component Strings and calculate ratio
fn calc(mut query: Vec<String>, map: &HashMap<String, usize>, nodes: &Vec<usize>, ratios: &Vec<f64>) -> f64 {
    if query.len() < 2 {
        panic!("less than two variables found in query {:?}", query);
    }
    let b = query.pop().unwrap();
    let a = query.pop().unwrap();
    if let Some(&a_index) = map.get(&a) {
        if let Some(&b_index) = map.get(&b) {
            if nodes[a_index] == nodes[b_index] {
                return ratios[a_index] / ratios[b_index];
            }
        }
    }
    -1.0
}

pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut nodes: Vec<usize> = Vec::new();
    let mut ratios: Vec<f64> = Vec::new();
    for (equation, value) in equations.into_iter().zip(values.into_iter()) {
        insert_eq(equation, value, &mut map, &mut nodes, &mut ratios);
    }
    for i in 0..nodes.len() {
        compress(i, &mut nodes, &mut ratios);
    }
    queries.into_iter().map(|query| calc(query, &map, &nodes, &ratios)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("c")],
                    ],
                vec![2.0, 3.0],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("a"), String::from("e")],
                    vec![String::from("a"), String::from("a")],
                    vec![String::from("x"), String::from("x")],
                ],
                vec![6.0, 0.5, -1.0, 1.0, -1.0],
            ),
            (
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("c"), String::from("d")],
                    vec![String::from("b"), String::from("d")],
                    ],
                vec![2.0, 2.0, 2.0],
                vec![
                    vec![String::from("a"), String::from("a")],
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("a"), String::from("d")],
                    vec![String::from("b"), String::from("b")],
                    vec![String::from("b"), String::from("c")],
                    vec![String::from("b"), String::from("d")],
                    vec![String::from("c"), String::from("c")],
                    vec![String::from("c"), String::from("d")],
                    vec![String::from("d"), String::from("d")],
                ],
                vec![1.0, 2.0, 2.0, 4.0, 1.0, 1.0, 2.0, 1.0, 2.0, 1.0],
            ),
            (
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("e"), String::from("f")],
                    vec![String::from("b"), String::from("e")],
                    ],
                vec![3.4, 1.4, 2.3],
                vec![
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("a"), String::from("f")],
                    vec![String::from("f"), String::from("f")],
                    vec![String::from("e"), String::from("e")],
                    vec![String::from("c"), String::from("c")],
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("f"), String::from("e")],
                ],
                vec![0.29412, 10.94800, 1.0, 1.0, -1.0, -1.0, 0.71429],
            ),
        ];
        for (equations, values, queries, expected) in test_tuples {
            assert_eq!(calc_equation(equations, values, queries), expected);
        }
    }
}

// #1 solution for comparison - uses DFS
// pub fn calc_equation(
//     equations: Vec<Vec<String>>,
//     values: Vec<f64>,
//     queries: Vec<Vec<String>>,
// ) -> Vec<f64> {
//     let mut graph: HashMap<&String, Vec<(f64, &String)>> = HashMap::new();
//     for i in 0..equations.len() {
//         (*graph.entry(&equations[i][0]).or_insert(vec![]))
//             .push((values[i], &equations[i][1]));
//         (*graph.entry(&equations[i][1]).or_insert(vec![]))
//             .push((1.0 / values[i], &equations[i][0]));
//     }
//     queries
//         .into_iter()
//         .map(|v| {
//             let mut seen = HashSet::new();
//             let mut q: VecDeque<(f64, &String)> = VecDeque::new();
//             if let Some(edges) = graph.get(&v[0]) {
//                 for edge in edges {
//                     q.push_back(edge.clone());
//                 }
//             }
//             while let Some((val, s)) = q.pop_front() {
//                 seen.insert(s.to_owned());
//                 if s == &v[1] {
//                     return val;
//                 } else {
//                     for edge in graph.get(&s).unwrap_or(&vec![]) {
//                         if !seen.contains(edge.1) {
//                             q.push_back((val * edge.0, edge.1));
//                         }
//                     }
//                 }
//             }
//             -1.0
//         })
//         .collect()
// }