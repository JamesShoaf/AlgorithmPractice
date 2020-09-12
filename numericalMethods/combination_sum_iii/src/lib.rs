/* 
Find all possible combinations of k numbers that add up to a number n, given that only numbers from
1 to 9 can be used and each combination should be a unique set of numbers.
*/

fn helper(k : &mut i32, n: &mut i32, stack: &mut Vec<i32>, output: &mut Vec<Vec<i32>>) {
    if *k == 0 && *n == 0 { output.push(stack.clone()); }
    else if *k > 0 && *n > 0 {
        *k -= 1;
        for i in 1 + if stack.len() > 0 { stack[stack.len() - 1] } else { 0 }..10 {
            *n -= i;
            stack.push(i);
            helper(k, n, stack, output);
            stack.pop();
            *n += i;
        }
        *k += 1;
    }
}

fn combination_sum3(mut k: i32, mut n: i32) -> Vec<Vec<i32>> {
    let mut output: Vec<Vec<i32>> = Vec::new();
    let mut stack: Vec<i32> = Vec::new();
    helper(&mut k, &mut n, &mut stack, &mut output);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (1, 10, Vec::new()),
            (2, 10, vec![vec![1, 9], vec![2, 8], vec![3, 7], vec![4, 6]]),
            (3, 7, vec![vec![1, 2, 4]]),
            (3, 9, vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]),
            (10, 50, Vec::new()),
        ];
        for (k, n, expected) in test_tuples {
            assert_eq!(combination_sum3(k, n), expected);
        }
    }
}
