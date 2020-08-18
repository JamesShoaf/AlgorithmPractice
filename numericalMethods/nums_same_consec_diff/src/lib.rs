/*
Return all non-negative integers of length N such that the absolute difference between every two consecutive digits is K.

Note that every number in the answer must not have leading zeros except for the number 0 itself. For example, 01 has one leading zero and is invalid, but 0 is valid.

You may return the answer in any order.

    1 <= N <= 9
    0 <= K <= 9

*/

fn nums_same_consec_diff(n:i32, k:i32) -> Vec<i32> {
    // generate mapping of nums k away from each digit
    let mut map: Vec<Vec<i32>> = vec![Vec::new(); 10];
    for i in 0..10 {
        if i - k >= 0 { map[i as usize].push(i - k );}
        if i + k <= 9 && k != 0 { map[i as usize].push(i + k);}
    }
    let mut queue = Vec::new();
    // if n == 1, include 0 in the starting set since 0 is valid.
    if n <= 1 { queue.push(0); }
    for i in 1..10 { queue.push(i); }
    let mut current_length = 1;
    // level order traversal of possible suffixes
    while queue.len() > 0 && current_length < n {
        let mut next_queue = Vec::new();
        for i in 0..queue.len() {
            let current = queue[i];
            let next_chars = &map[(current % 10) as usize];
            let current = current * 10;
            for j in 0..next_chars.len() {
                next_queue.push(current + next_chars[j]);
            }
        }
        queue = next_queue;
        current_length += 1;
    }
    queue
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (1, 9, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            (3, 0, vec![111, 222, 333, 444, 555, 666, 777, 888, 999]),
            (3, 7, vec![181, 292, 707, 818, 929]),
            (2, 1, vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98]),
            (9, 9, vec![909090909]),
        ];
        for (n, k, expected) in test_tuples {
            assert_eq!(nums_same_consec_diff(n, k), expected);
        }
    }
}
