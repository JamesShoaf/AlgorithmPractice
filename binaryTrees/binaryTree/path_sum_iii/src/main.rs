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
impl Solution {
    fn recursive(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> i32 {
        let mut counter: i32 = 0;
        if let Some(rc) = root {
            let node = rc.borrow();
            if node.val == target { counter += 1; }
            let new_target = target - node.val;
            counter += Solution::recursive(node.left.clone(), new_target)
                + Solution::recursive(node.right.clone(), new_target);
        }
        counter
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut counter: i32 = 0;
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];
        while !stack.is_empty() {
            if let Some(option) = stack.pop() {
                counter += Solution::recursive(option.clone(), sum);
                if let Some(rc) = option.clone() {
                    let node = rc.borrow();
                    stack.push(node.left.clone());
                    stack.push(node.right.clone());
                }
            }
        }
        counter
    }
}


fn main() {
    println!("Hello, world!");
}
