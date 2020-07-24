struct Solution {}

impl Solution {
    fn recursive_helper(graph: &Vec<Vec<i32>>, output: &mut Vec<Vec<i32>>, path: Vec<i32>, node: usize) {
        if node == graph.len() - 1 {
            output.push(path);
        } else {
            for edge in graph[node].iter() {
                let mut next_path = path.clone();
                next_path.push(*edge);
                Self::recursive_helper(&graph, output, next_path, *edge as usize);
            }
        }
    }
    pub fn all_paths_source_target (graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut output: Vec<Vec<i32>> = Vec::new();
        Self::recursive_helper(&graph, &mut output, vec![0], 0);
        output
    }
}

fn main() {
    let test = vec![vec![1, 2], vec![3], vec![3], Vec::new()];
    let output = Solution::all_paths_source_target(test);
    println!("{:#?}", output);
}
