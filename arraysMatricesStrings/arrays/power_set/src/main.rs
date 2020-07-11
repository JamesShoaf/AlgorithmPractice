struct Solution {}

impl Solution {
    fn power_set(array: Vec<i32>) -> Vec<Vec<i32>> {
        let mut stack: Vec<(Vec<i32>, usize)> = vec![(Vec::new(), 0)];
        let mut output: Vec<Vec<i32>> = vec![Vec::new()];
        while stack.len() > 0 {
            let (parent, index) = stack.pop().unwrap();
            if let Some(child) = Solution::recursive_helper(&array, &mut stack, parent, index) {
                output.push(child);
            }
        }
        output
    }
    fn recursive_helper(array: &Vec<i32>, stack: &mut Vec<(Vec<i32>, usize)>,
        parent: Vec<i32>, index: usize) -> Option<Vec<i32>> {
        return if index >= array.len() { None }
        else {
            stack.push((parent.clone(), index + 1));
            let mut child = parent.clone();
            child.push(array[index]);
            stack.push((child.clone(), index + 1));
            Some(child)
        }
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    let output = Solution::power_set(nums);
    for vec in output {
        println!("{:#?}", vec);
    }
}
