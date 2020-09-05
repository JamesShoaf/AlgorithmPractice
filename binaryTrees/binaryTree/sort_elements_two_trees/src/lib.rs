use std::rc::Rc;
use std::cell::RefCell;

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

fn get_elements(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut output = Vec::new();
    let mut node_stack = Vec::new();
    if let Some(node) = root {
        node_stack.push(node.clone());
    }
    while !node_stack.is_empty() {
        if let Some(node) = node_stack.pop() {
            let node = node.borrow();
            output.push(node.val);
            if let Some(left) = &node.left {
                node_stack.push(left.clone());
            }
            if let Some(left) = &node.right {
                node_stack.push(left.clone());
            }
        }
    }
    output
}

fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut output = Vec::new();
    output.append(&mut get_elements(&root1));
    output.append(&mut get_elements(&root2));
    output.sort();
    output
}

