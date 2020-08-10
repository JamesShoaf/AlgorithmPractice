struct Solution {}

impl Solution {
    pub fn oranges_rotting(mut matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.len() == 0 || matrix[0].len() == 0 { return 0; }
        let mut rotten: Vec<(i32, i32)> = Vec::new();
        for x in 0..matrix.len() {
            for y in 0..matrix[0].len() {
                if matrix[x][y] == 2 { rotten.push((x as i32, y as i32)); }
            }
        }

        let mut time: i32 = 0;
        loop {
            let mut next_to_rot: Vec<(usize, usize)> = Vec::new();
            while !rotten.is_empty() {
                let (x, y) = rotten.pop().unwrap();
                for (new_x, new_y) in vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)] {
                    if new_x >= 0 && new_x < matrix.len() as i32 && new_y >= 0 && new_y < matrix[0].len() as i32 {
                        next_to_rot.push((new_x as usize, new_y as usize));
                    }
                }
            }
            for (x, y) in next_to_rot {
                if matrix[x][y] == 1 {
                    matrix[x][y] = 2;
                    rotten.push((x as i32, y as i32));
                }
            }
            if rotten.is_empty() { break; } else { time += 1; }
        }

        for x in 0..matrix.len() {
            for y in 0..matrix[0].len() {
                if matrix[x][y] == 1 { return -1; }
            }
        }
        time
    }
}


fn main() {
    let test_tuples = vec![
        (vec![
            vec![2, 1, 1],
            vec![1, 1, 0],
            vec![0, 1, 1],
        ],
        4),
        (vec![
            vec![2, 1, 1],
            vec![0, 1, 1],
            vec![1, 0, 1],
        ],
        -1),
        (vec![
            vec![0, 2],
        ],
        0),
        (vec![
            vec![2, 2, 2, 2],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
        ],
        3),
        (vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
        ],
        3),
        (vec![
            vec![2, 1, 1, 1],
            vec![2, 1, 1, 1],
            vec![2, 1, 1, 1],
            vec![2, 1, 1, 1],
        ],
        3),
        (vec![
            vec![1, 1, 1, 2],
            vec![1, 1, 1, 2],
            vec![1, 1, 1, 2],
            vec![1, 1, 1, 2],
        ],
        3),
    ];
    for (matrix, expected) in test_tuples {
        assert_eq!(Solution::oranges_rotting(matrix), expected);
    }
}
