use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn top_sort(num_nodes: i32, dependencies: Vec<Vec<i32>>) -> Vec<i32> {
        // convert dependency edges to an array of nodes
        let mut nodes: Vec<Vec<i32>> = Vec::new();
        for _ in 0..num_nodes { nodes.push(Vec::new()); }
        for dependency in dependencies {
            let dependent = dependency[0];
            let required = dependency[1];
            nodes[required as usize].push(dependent);
        }
        // keep a set of visited nodes
        let mut visited: HashSet<i32> = HashSet::new();
        // a set of nodes in the current chain
        let mut current_path: HashSet<i32> = HashSet::new();
        // a stack of nodes to revisit
        let mut node_stack: Vec<(i32, usize)> = Vec::new();
        // and an output vector to put fully explored nodes in
        let mut output: Vec<i32> = Vec::new();

        // choose the first unvisited node from the list of nodes
        for node in 0..num_nodes {
            if !visited.contains(&node) {
                node_stack.push((node, 0));
                while node_stack.len() > 0 {
                    let mut unvisited_dependencies = false;
                    let (node, starting_index) = node_stack.pop().unwrap();
                    current_path.insert(node);
                    let child_count = nodes[node as usize].len();
                    if starting_index <= child_count {
                        // choose the first unvisited dependency
                        for index in starting_index..child_count {
                            let next_dependency = nodes[node as usize][index];
                            if !visited.contains(&next_dependency) {
                                // push the current node and the index of its next dependency to the stack
                                node_stack.push((node, index + 1));
                                node_stack.push((next_dependency, 0));
                                unvisited_dependencies = true;
                                // if one of that node's dependencies has already been visited in the current chain
                                // return an empty vector (the graph is not a Directed Acyclic Graph)
                                if current_path.contains(&next_dependency) { return Vec::new(); }
                                break;
                            }
                        }
                    }
                    // if the current node has no unvisited dependencies
                    if !unvisited_dependencies {
                        // push it to the output
                        output.push(node);
                        // remove it from the path
                        current_path.remove(&node);
                        // and mark it visited
                        visited.insert(node);
                    }
                }
            }
        }
        // reverse the output and return it
        output.reverse();
        output
    }
}

fn main() {
    let n = 3;
    let dependencies = vec![vec![1, 0], vec![2, 1], vec![0, 2]];
    for node in Solution::top_sort(n, dependencies) {
        println!("{}", node);
    }
}
