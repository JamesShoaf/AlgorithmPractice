/* 
On a 2-dimensional grid, there are 4 types of squares:

    1 represents the starting square.  There is exactly one starting square.
    2 represents the ending square.  There is exactly one ending square.
    0 represents empty squares we can walk over.
    -1 represents obstacles that we cannot walk over.

Return the number of 4-directional walks from the starting square to the ending square, that walk
over every non-obstacle square exactly once.
*/
use std::collections::HashSet;

fn get_neighbors(x: usize, y: usize, mat: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if x < mat.len() - 1 {
        neighbors.push((x + 1, y));
    }
    if y < mat[x].len() - 1 {
        neighbors.push((x, y + 1));
    }
    neighbors
}

fn backtrack(x: usize, y: usize, mat: &Vec<Vec<i32>>, set: &mut HashSet<(usize, usize)>, count: &mut i32) {
    if mat[x][y] == 2 {
        for x in 0..mat.len() {
            for y in 0..mat[x].len() {
                if mat[x][y] != -1 && !set.contains(&(x, y)) {
                    return;
                }
            }
        }
        *count += 1;
        return;
    }
    for (w, z) in get_neighbors(x, y, mat) {
        if mat[w][z] != -1 && !set.contains(&(w, z)) {
            set.insert((w, z));
            backtrack(w, z, mat, set, count);
            set.remove(&(w, z));
        }
    }
}

pub fn unique_paths_iii(mat: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..mat.len() {
        for y in 0..mat[x].len() {
            if mat[x][y] == 1 {
                set.insert((x, y));
                backtrack(x, y, &mat, &mut set, &mut count);
                return count;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                vec![
                    vec![1, 2],
                ],
                1,
            ),
            (
                vec![
                    vec![1, 0, 0, 2],
                ],
                1,
            ),
            (
                vec![
                    vec![1, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 2, -1],
                ],
                2,
            ),
            (
                vec![
                    vec![1, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 2],
                ],
                4,
            ),
            (
                vec![
                    vec![1, 0],
                    vec![0, 2],
                ],
                0,
            ),
        ];
        for (mat, expected) in test_tuples {
            assert_eq!(unique_paths_iii(mat), expected);
        }
    }
}
