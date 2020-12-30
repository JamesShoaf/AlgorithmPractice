use std::cmp;
use std::collections::HashSet;

type Matrix = Vec<Vec<Option<i32>>>;

fn setup(adj_mat: &Matrix, start: usize, memo: &mut Matrix) {
    for i in (0..adj_mat.len()).filter(|&i| i != start) {
        memo[i][1 << start | 1 << i] = adj_mat[start][i];
    }
}

// generates all numbers up to 2^n - 1 with r set bits
fn recursive_combinations(
    output: &mut Vec<usize>,
    dead_ends: &mut HashSet<(usize, usize)>,
    curr: usize,
    r: usize,
    i: usize,
    n: usize,
) -> bool {
    if dead_ends.contains(&(r, i)) {
        return false;
    }
    if r == 0 {
        output.push(curr);
        return true;
    }
    if i == n {
        return false;
    }
    if !recursive_combinations(output, dead_ends, curr | 1 << i, r - 1, i + 1, n) {
        dead_ends.insert((r, i));
        return false;
    }
    recursive_combinations(output, dead_ends, curr, r, i + 1, n);
    true
}

fn combinations(r: usize, n: usize) -> Vec<usize> {
    let mut output = Vec::new();
    let mut dead_ends: HashSet<(usize, usize)> = HashSet::new();
    recursive_combinations(&mut output, &mut dead_ends, 0, r, 0, n);
    output
}

fn in_subset(i: usize, subset: usize) -> bool {
    subset & 1 << i != 0
}

fn solve(n: usize, start: usize, memo: &mut Matrix) {
    for r in 3..=n {
        for subset in combinations(r, n) {
            if !in_subset(start, subset) {
                continue;
            }
            for next in (0..n).filter(|&i| in_subset(i, subset) && i != start) {
                let state = subset ^ 1 << next;
                let mut min_distance = None;
                for end in (0..n).filter(|&i| !in_subset(i, subset) && i != start && i != next) {
                    if let Some(length_to_end) = memo[end][state] {
                        if let Some(end_to_next) = memo[end][next] {
                            if let Some(min_distance) = min_distance.as_mut() {
                                *min_distance =
                                    cmp::min(*min_distance, length_to_end + end_to_next);
                            } else {
                                min_distance = Some(length_to_end + end_to_next);
                            }
                        }
                    }
                }
                memo[next][subset] = min_distance;
            }
        }
    }
}

fn completed_tour(n: usize) -> usize {
    if n == 32 {
        usize::MAX
    } else {
        2_usize.pow(n as u32) - 1
    }
}

fn find_min_cost(adj_mat: &Matrix, start: usize, memo: &Matrix) -> Option<i32> {
    let n = adj_mat.len();
    let end_state = completed_tour(n);
    (0..n)
        .filter(|&i| i != start)
        .filter(|&i| memo[i][end_state].is_some())
        .filter(|&i| adj_mat[i][start].is_some())
        .map(|i| *memo[i][end_state].as_ref().unwrap() + *adj_mat[i][start].as_ref().unwrap())
        .min()
}

fn find_optimal_tour(adj_mat: &Matrix, start: usize, memo: &Matrix) -> Option<Vec<usize>> {
    let n = adj_mat.len();
    let mut last_index = start;
    let mut state = completed_tour(n);
    let mut tour = vec![start];

    for _ in (1..n).rev() {
        let (_, index) = (0..n)
            .filter(|&i| {
                i != start
                    && in_subset(i, state)
                    && memo[i][state].is_some()
                    && adj_mat[i][last_index].is_some()
            })
            .map(|i| {
                (
                    *memo[i][state].as_ref().unwrap() + *adj_mat[i][last_index].as_ref().unwrap(),
                    i,
                )
            })
            .min()?;
        tour.push(index);
        state ^= 1 << index;
        last_index = index;
    }
    tour.push(start);
    Some(tour)
}

pub fn traveling_salesman(adj_mat: &Matrix, start: usize) -> Option<(i32, Vec<usize>)> {
    let n = adj_mat.len();
    assert!(n > 0 && n <= 32, "node count must be in range [1, 32]");
    assert!(
        adj_mat.iter().all(|row| row.len() == n),
        "dimension mismatch: matrix must be n x n"
    );
    assert!(n > start, "starting node must be in range [0, n)");
    let mut memo: Matrix = vec![vec![None; 2_usize.pow(n as u32)]; n];
    setup(adj_mat, start, &mut memo);
    solve(n, start, &mut memo);
    if let Some(min_cost) = find_min_cost(&adj_mat, start, &memo) {
        if let Some(tour) = find_optimal_tour(adj_mat, start, &memo) {
            return Some((min_cost, tour));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_combinations() {
        let test_tuples = vec![
            (0, 32, vec![0]),
            (0, 4, vec![0]),
            (1, 4, vec![1, 2, 4, 8]),
            (2, 4, vec![3, 5, 9, 6, 10, 12]),
            (3, 4, vec![14, 13, 11, 7]),
            (4, 4, vec![15]),
        ];
        for (r, n, mut expected) in test_tuples {
            expected.sort();
            let mut actual = combinations(r, n);
            actual.sort();
            assert_eq!(actual, expected);
        }
    }
}
