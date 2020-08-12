/* Given a non-negative index k where k ≤ 33, return the kth index row of the Pascal's triangle. */
fn get_row(row_index: i32) -> Vec<i32> {
    let mut output: Vec<i32> = vec![1];
    for _ in 0..row_index {
        if output.len() > 1 {
            for i in (0..=output.len() - 2).rev() {
                output[i + 1] += output[i];
            }
        }
        output.push(1);
    }
    output
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (0, vec![1]),
            (1, vec![1, 1]),
            (2, vec![1, 2, 1]),
            (3, vec![1, 3, 3, 1]),
            (33, vec![1, 33, 528, 5456, 40920, 237336, 1107568, 4272048, 13884156, 38567100, 92561040, 193536720, 354817320, 573166440, 818809200, 1037158320, 1166803110, 1166803110, 1037158320, 818809200, 573166440, 354817320, 193536720, 92561040, 38567100, 13884156, 4272048, 1107568, 237336, 40920, 5456, 528, 33, 1]),
        ];
        for (k, expected) in test_tuples {
            assert_eq!(super::get_row(k), expected, );
        }
    }
}
