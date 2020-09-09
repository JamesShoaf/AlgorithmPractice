/* 
Given a binary tree, each node has value 0 or 1.  Each root-to-leaf path represents a binary number
starting with the most significant bit.  For example, if the path is 0 -> 1 -> 1 -> 0 -> 1, then
this could represent 01101 in binary, which is 13.

For all leaves in the tree, consider the numbers represented by the path from the root to that leaf.

Return the sum of these numbers.
*/

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

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut acc: i32) -> i32 {
    if let Some(inner) = node {
        let inner = inner.borrow();
        // shift the previous value left by 1 and add 1 if the current node is 1
        acc <<= 1;
        acc += inner.val;
        // if the current node is a leaf, return the accumulator
        if inner.left.is_none() && inner.right.is_none() {
            return acc;
        }
        // otherwise, pass the accumulator to a recursive call on left and right
        return dfs(&inner.left, acc) + dfs(&inner.right, acc);
    }
    // return 0 if the node is null
    0
}

fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    dfs(&root, 0)
}