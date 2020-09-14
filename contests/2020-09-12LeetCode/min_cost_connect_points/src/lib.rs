/* 
You are given an array points representing integer coordinates of some points on a 2D-plane, where points[i] = [xi, yi].

The cost of connecting two points [xi, yi] and [xj, yj] is the manhattan distance between them: |xi - xj| + |yi - yj|, where |val| denotes the absolute value of val.

Return the minimum cost to make all points connected. All points are connected if there is exactly one simple path between any two points.
*/

// recursively calls itself from the initial node then compresses the path from node to root
fn find(node: usize, nodes: &mut Vec<usize>) -> usize {
    if nodes[node] != node {
        nodes[node] = find(nodes[node], nodes);
    }
    nodes[node]
}

// returns false if the two nodes have the same root, or unites them and returns true
fn union(first: usize, second: usize, nodes: &mut Vec<usize>) -> bool {
    let (first, second) = (find(first, nodes), find(second, nodes));
    if first == second { return false; }
    // which root becomes parent is arbitrary if path compression occurs later
    nodes[second] = first;
    true
}

// generate vec of distances between all discrete points
fn find_all_distances(points: Vec<Vec<i32>>) -> Vec<(i32, usize, usize)> {
    (0..points.len())
        .map(|i| find_distances_from_point(i, &points))
        .flatten()
        .collect()
}

// add all distances from a point i to each other point to a vec
fn find_distances_from_point(i: usize, points: &Vec<Vec<i32>>) -> Vec<(i32, usize, usize)> {
    (i + 1..points.len())
        .map(|j| find_distance(i, j, points))
        .collect()
}

// generate a distance given a point i or j
fn find_distance(i: usize, j: usize, points: &Vec<Vec<i32>>) -> (i32, usize, usize) {
    ((points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs(), i, j)
}

// Kruskal's algorithm, uses a MinHeap and a Union-Find to assemble the Minimum Spanning Tree
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    // each point initially is its own root
    let mut nodes = (0..points.len()).collect();
    let mut distances = find_all_distances(points);
    distances.sort();
    distances.iter()
        .filter(|&&(_, i, j)| union(i, j, &mut nodes))
        .fold(0, |acc, &(dist, _, _)| { acc + dist })
} 

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), 0),
            (vec![vec![0, 0]], 0),
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
