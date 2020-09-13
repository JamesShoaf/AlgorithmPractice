/* 
You are given an array points representing integer coordinates of some points on a 2D-plane, where points[i] = [xi, yi].

The cost of connecting two points [xi, yi] and [xj, yj] is the manhattan distance between them: |xi - xj| + |yi - yj|, where |val| denotes the absolute value of val.

Return the minimum cost to make all points connected. All points are connected if there is exactly one simple path between any two points.
*/

// recursively calls itself from the initial node then compresses the path from node to root
fn find(node: usize, nodes: &mut Vec<usize>) -> usize {
    if nodes[node] == node { return node; }
    nodes[node] = find(nodes[node], nodes);
    nodes[node]
}

// returns false if the two nodes have the same root, or unites them and returns true
fn union(first: usize, second: usize, nodes: &mut Vec<usize>) -> bool {
    let first_root = find(first, nodes);
    let second_root = find(second, nodes);
    if first_root == second_root { return false; }
    nodes[second_root] = first_root;
    true
}

fn find_distance(i: usize, j: usize, points: &Vec<Vec<i32>>) -> (i32, usize, usize) {
    ((points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs(), i, j)
}

// Kruskal's algorithm, uses a MinHeap and a Union-Find to assemble the Minimum Spanning Tree
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let mut min_cost = 0;
    let mut nodes: Vec<usize> = Vec::new();
    let mut distances: Vec<(i32, usize, usize)> = Vec::new();

    // each point initially is its own root
    for i in 0..points.len() { nodes.push(i); }
    
    // add all n^2 distances to the heap
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            distances.push(find_distance(i, j, &points));
        }
    }

    // sort distances in ascending order
    distances.sort();
    for &(dist, i, j) in distances.iter() {
        if union(i, j, &mut nodes) {
            min_cost += dist;
        }
    }

    min_cost
} 

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]], 20),
            (vec![vec![3, 12], vec![-2, 5], vec![-4, 1]], 18),
            (vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![-1, 1]], 4),
            (vec![vec![-1000000, -1000000], vec![1000000, 1000000]], 4000000),
            (vec![vec![0, 0], vec![2, 3], vec![6, 8], vec![11, 20]], 31),
        ];
        for (points, expected) in test_tuples {
            assert_eq!(min_cost_connect_points(points), expected);
        }
    }
}
