fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
    let start = rounds[0];
    let finish = rounds[rounds.len() - 1];
    let mut output = Vec::new();
    if finish >= start {
        for i in start..=finish {
            output.push(i);
        }
        return output;
    }
    for i in 1..=finish {
        output.push(i);
    }
    for i in start..=n {
        output.push(i);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (4, vec![1, 3, 1, 2], vec![1, 2]),
            (2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2], vec![2]),
            (7, vec![1, 3, 5, 7], vec![1, 2, 3, 4, 5, 6, 7]),
            (3, vec![3, 2, 1, 2, 1, 3, 2, 1, 2, 1, 3, 2, 3, 1], vec![1, 3]),
        ];
        for (n, rounds, expected) in test_tuples {
            assert_eq!(most_visited(n, rounds), expected);
        }
    }
}
