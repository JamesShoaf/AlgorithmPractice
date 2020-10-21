/* 
We are given an array asteroids of integers representing asteroids in a row.

For each asteroid, the absolute value represents its size, and the sign represents its direction
(positive meaning right, negative meaning left). Each asteroid moves at the same speed.

Find out the state of the asteroids after all collisions. If two asteroids meet, the smaller one
will explode. If both are the same size, both will explode. Two asteroids moving in the same
direction will never meet.
*/

pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut res: Vec<i32> = Vec::new();
    for val in asteroids {
        if val <= 0 {
            loop {
                if stack.is_empty() {
                    res.push(val);
                    break;
                }
                // short circuit to only pop the stack if the top of the stack is >= to val.abs()
                if val.abs() < stack[stack.len() - 1] || val.abs() == stack.pop().unwrap() {
                    break;
                }
            }
        } else {
            stack.push(val);
        }
    }
    res.append(&mut stack);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![5, 10, -5], vec![5, 10]),
            (vec![8, -8], Vec::new()),
            (vec![10, 2, -5], vec![10]),
            (vec![-2, -1, 1, 2], vec![-2, -1, 1, 2]),
        ];
        for (asteroids, expected) in test_tuples {
            assert_eq!(asteroid_collision(asteroids), expected);
        }
    }
}
