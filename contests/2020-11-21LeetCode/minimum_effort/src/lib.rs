/*
You are given an array tasks where tasks[i] = [actuali, perceivedi]:

    actuali is the actual amount of energy you spend to finish the ith task.
    perceivedi is the minimum amount of energy you require to begin the ith task.

For example, if the task is [10, 12] and your current energy is 11, you cannot start this task.
However, if your current energy is 13, you can complete this task, and your energy will be 3 after
finishing it.

You can finish the tasks in any order you like.

Return the minimum initial amount of energy you will need to finish all the tasks.

Constraints
    1 <= tasks.length <= 10^5
    1 <= actual​i <= minimumi <= 10^4

*/

use std::cmp;

pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
    let max_perceived = tasks.iter().map(|vec| vec[1]).max().unwrap_or(0);
    let min_difference = tasks.iter().map(|vec| vec[1] - vec[0]).min().unwrap_or(0);
    let actual_effort: i32 = tasks.iter().map(|vec| vec[0]).sum();
    cmp::max(actual_effort + min_difference, max_perceived)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
