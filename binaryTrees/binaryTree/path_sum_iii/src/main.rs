// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
impl Solution {
    // O(N) time, O(N) space
    fn recursive(node: &Option<Rc<RefCell<TreeNode>>>, sum: i32, target: i32, map: &mut HashMap<i32, i32>) -> i32 {
        let mut counter: i32 = 0;
        if let Some(node) = node {
            let node = node.borrow();
            let curr_sum = sum + node.val;
            let diff = curr_sum - target;
            if let Some(count) = map.get(&diff) { counter += *count; }
            *map.entry(curr_sum).or_insert(0) += 1;
            counter += Solution::recursive(&node.left, curr_sum, target, map);
            counter += Solution::recursive(&node.right, curr_sum, target, map);
            *map.get_mut(&curr_sum).unwrap() -= 1;
        }
        counter 
    }    

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut sums: HashMap<i32, i32> = HashMap::new();
        sums.insert(0, 1);
        Solution::recursive(&root, 0, sum, &mut sums)
    }

    // O(N^2) time, O(1) space
    // fn recursive(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> i32 {
    //     let mut counter: i32 = 0;
    //     if let Some(rc) = root {
    //         let node = rc.borrow();
    //         if node.val == target { counter += 1; }
    //         let new_target = target - node.val;
    //         counter += Solution::recursive(node.left.clone(), new_target)
    //             + Solution::recursive(node.right.clone(), new_target);
    //     }
    //     counter
    // }

    // pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
    //     let mut counter: i32 = 0;
    //     let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];
    //     while !stack.is_empty() {
    //         if let Some(option) = stack.pop() {
    //             counter += Solution::recursive(option.clone(), sum);
    //             if let Some(rc) = option.clone() {
    //                 let node = rc.borrow();
    //                 stack.push(node.left.clone());
    //                 stack.push(node.right.clone());
    //             }
    //         }
    //     }
    //     counter
    // }
}


fn main() {
    let mut root = TreeNode::new(1);
    let mut left = TreeNode::new(2);
    let left_left = TreeNode::new(4);
    let left_right = TreeNode::new(6);
    left.left = Some(Rc::new(RefCell::new(left_left)));
    left.right = Some(Rc::new(RefCell::new(left_right)));
    root.left = Some(Rc::new(RefCell::new(left)));
    let mut right = TreeNode::new(3);
    let right_left = TreeNode::new(5);
    let right_right = TreeNode::new(7);
    right.left = Some(Rc::new(RefCell::new(right_left)));
    right.right = Some(Rc::new(RefCell::new(right_right)));
    root.right = Some(Rc::new(RefCell::new(right)));
    let output = Solution::path_sum(Some(Rc::new(RefCell::new(root))), 9);
    println!("path sum {}", output);
}
