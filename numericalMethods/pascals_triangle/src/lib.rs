// Given a non-negative index k where k ≤ 33, return the kth index row of the Pascal's triangle.
// O(n^2) time, O(n) space
fn get_row(row_index: i32) -> Vec<i32> {
    if row_index <= 12 { return get_row_binomial(row_index); }
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

// O(n) time, O(n) space optimization for n <= 12 (because 13! > 2^31)
fn get_row_binomial(row_index: i32) -> Vec<i32> {
    let mut fac: Vec<i32> = vec![1];
    for i in 1..=row_index { fac.push(fac[i as usize - 1] * i); }
    let mut output: Vec<i32> = Vec::new();
    for i in 0..=(row_index + 1) / 2 {
        output.push(fac[row_index as usize] / (fac[i as usize] * fac[(row_index - i) as usize]));
    }
    for i in (0..row_index / 2).rev() { output.push(output[i as usize]); }
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
            (12, vec![1, 12, 66, 220, 495, 792, 924, 792, 495, 220, 66, 12, 1]),
            (33, vec![1, 33, 528, 5456, 40920, 237336, 1107568, 4272048, 13884156, 38567100, 92561040, 193536720, 354817320, 573166440, 818809200, 1037158320, 1166803110, 1166803110, 1037158320, 818809200, 573166440, 354817320, 193536720, 92561040, 38567100, 13884156, 4272048, 1107568, 237336, 40920, 5456, 528, 33, 1]),
        ];
        for (k, expected) in test_tuples {
            assert_eq!(super::get_row(k), expected, );
        }
    }
}
