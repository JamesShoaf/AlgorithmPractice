/* Given an array of distinct positive integers candidates and a target positive integer target,
return a list of all unique combinations of candidates where the chosen numbers sum to target. You
may return the combinations in any order.

The same number may be chosen from candidates an unlimited number of times. Two combinations are
unique if the frequency of at least one of the chosen numbers is different.
*/

fn helper(index: usize, nums: &Vec<i32>, target: &mut i32, stack: &mut Vec<i32>, output: &mut Vec<Vec<i32>>) {
    if *target == 0 {
        output.push(stack.clone());
    }
    for i in index..nums.len() {
        if nums[i] > *target {
            break;
        }
        *target -= nums[i];
        stack.push(nums[i]);
        helper(i, nums, target, stack, output);
        *target += stack.pop().unwrap();
    }
}

pub fn combination_sum(mut nums: Vec<i32>, mut target: i32) -> Vec<Vec<i32>> {
    nums.sort();
    let mut stack: Vec<i32> = Vec::new();
    let mut output: Vec<Vec<i32>> = Vec::new();
    helper(0, &nums, &mut target, &mut stack, &mut output);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                vec![2, 3, 6, 7],
                7,
                vec![
                    vec![2, 2, 3],
                    vec![7],
                ]
            ),
        ];
        for (nums, target, expected) in test_tuples {
            assert_eq!(combination_sum(nums, target), expected);
        }
    }
}
