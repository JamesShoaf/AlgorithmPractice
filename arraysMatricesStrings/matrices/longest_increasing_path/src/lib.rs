use std::collections::HashMap;
use std::cmp;

fn is_increasing(x: usize, y: usize, new_x: usize, new_y: usize, mat: &Vec<Vec<i32>>) -> bool {
    mat[new_x][new_y] > mat[x][y]
}

fn greater_neighbors(x: usize, y: usize, mat: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut output = Vec::new();
    if x > 0 && y < mat[x - 1].len() && is_increasing(x, y, x - 1, y, mat) { output.push((x - 1, y)); }
    if y > 0 && is_increasing(x, y, x, y - 1, mat) { output.push((x, y - 1)); }
    if x + 1 < mat.len() && y < mat[x + 1].len() && is_increasing(x, y, x + 1, y, mat) { output.push((x + 1, y)); }
    if y + 1 < mat[x].len() && is_increasing(x, y, x, y + 1, mat) { output.push((x, y + 1)); }
    output
}

fn helper(x: usize, y: usize, mat: &Vec<Vec<i32>>, memo: &mut HashMap<(usize, usize), i32>) -> i32 {
    if let Some(&prev) = memo.get(&(x, y)) {
        return prev;
    }
    let best_path = greater_neighbors(x, y, mat).iter()
        .fold(1, |acc, &(new_x, new_y)| cmp::max(acc, 1 + helper(new_x, new_y, mat, memo)));
    memo.insert((x, y), best_path);
    best_path
}

pub fn longest_increasing_path(mat: Vec<Vec<i32>>) -> i32 {
    let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
    (0..mat.len()).fold(0, |acc, x| cmp::max(acc, 
        (0..mat[x].len()).fold(0, |acc, y| cmp:: max(acc, helper(x, y, &mat, &mut memo)))
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![
                vec![9, 9, 4],
                vec![6, 6, 8],
                vec![2, 1, 1],
            ],
            4),
            (vec![
                vec![3, 4, 5],
                vec![3, 2, 6],
                vec![2, 2, 1],
            ],
            4),
            (vec![
                vec![5, 6, 7],
                vec![4],
                vec![3, 2, 1],
            ],
            7),
        ];
        for (mat, expected) in test_tuples {
            assert_eq!(longest_increasing_path(mat), expected);
        }
    }
}
