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

type Tree = Option<Rc<RefCell<TreeNode>>>;

/* 
Given the root of a binary tree, return the sum of every tree node's tilt.

The tilt of a tree node is the absolute difference between the sum of all left subtree node values
and all right subtree node values. If a node does not have a left child, then the sum of the left
subtree node values is treated as 0. The rule is similar if there the node does not have a right
child.
*/

struct Tilt {
    vals: i32,
    tilts: i32,
}

fn recursive_helper(node: &Tree) -> Tilt {
    if let Some(inner) = node {
        let inner = inner.borrow();
        let left = recursive_helper(&inner.left);
        let right = recursive_helper(&inner.right);
        return Tilt {
            vals: left.vals + right.vals + inner.val,
            tilts: left.tilts + right.tilts + (left.vals - right.vals).abs(),
        }
    }
    Tilt {
        vals: 0,
        tilts: 0,
    }
}

pub fn find_tilt(root: Tree) -> i32 {
    recursive_helper(&root).tilts
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
