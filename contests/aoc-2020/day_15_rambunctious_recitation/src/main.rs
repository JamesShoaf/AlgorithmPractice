fn main() {
    let input = vec![8, 0, 17, 4, 1, 12];
    println!("memory game: {}", memory_game(&input, 2020));
    println!("memory game: {}", memory_game(&input, 30000000));
}

use std::collections::HashMap;

fn memory_game(vec: &Vec<usize>, target: usize) -> usize {
    let mut last_spoken = target;
    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..vec.len() {
        if i > 0 {
            map.insert(last_spoken, i - 1);
        }
        last_spoken = vec[i];
    }
    for i in vec.len()..target {
        if let Some(j) = map.insert(last_spoken, i - 1) {
            last_spoken = i - j - 1;
        } else {
            last_spoken = 0;
        }
    }
    last_spoken
}

#[test]
fn test_memory_game() {
    let test_tuples = vec![
        (vec![1, 3, 2], 1),
        (vec![2, 1, 3], 10),
        (vec![1, 2, 3], 27),
        (vec![2, 3, 1], 78),
        (vec![3, 2, 1], 438),
        (vec![3, 1, 2], 1836),
    ];
    for (start, expected) in test_tuples {
        assert_eq!(memory_game(&start, 2020), expected);
    }
}
