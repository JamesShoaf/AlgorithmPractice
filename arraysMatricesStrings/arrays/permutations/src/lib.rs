use std::collections::HashSet;

// Heap's algorithm + HashSet to guarantee uniqueness

fn helper(nums: &mut Vec<i32>, size: usize, set: &mut HashSet<Vec<i32>>) {
    if size == 1 {
        set.insert(nums.clone());
        return;
    }
    for i in 0..size {
        helper(nums, size - 1, set);
        nums.swap(size - 1, i * (1 ^ size % 2));
    }
}

pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        return Vec::new();
    }
    let mut set: HashSet<Vec<i32>> = HashSet::new();
    let n = nums.len();
    helper(&mut nums, n, &mut set);
    set.into_iter().collect()
}

/*
#1 answer for comparison

fn backtrack(
    n: usize,
    nums: &Vec<i32>,
    cur: &mut Vec<i32>,
    ans: &mut Vec<Vec<i32>>,
    start: usize,
    vis: &mut Vec<bool>,
) {
    if start == n {
        ans.push(cur.clone());
        return;
    }

    for i in 0..nums.len() {
        if vis[i] || (i > 0 && nums[i] == nums[i - 1] && !vis[i - 1]) {
            continue;
        }

        cur.push(nums[i]);
        vis[i] = true;
        Self::backtrack(n, nums, cur, ans, start + 1, vis);
        vis[i] = false;
        cur.pop();
    }
}

pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let n = nums.len();
    let mut vis = vec![false; n];
    nums.sort();
    Self::backtrack(n, &nums, &mut Vec::new(), &mut ans, 0, &mut vis);
    ans
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), Vec::new()),
            (vec![1], vec![vec![1]]),
            (vec![1, 1], vec![vec![1, 1]]),
            (vec![1, 2], vec![vec![1, 2], vec![2, 1]]),
            (
                vec![1, 1, 2],
                vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]],
            ),
            (
                vec![3, 3, 0, 3],
                vec![
                    vec![0, 3, 3, 3],
                    vec![3, 0, 3, 3],
                    vec![3, 3, 0, 3],
                    vec![3, 3, 3, 0],
                ],
            ),
        ];
        for (nums, expected) in test_tuples {
            let mut actual = permute_unique(nums);
            actual.sort();
            assert_eq!(actual, expected);
        }
    }
}
