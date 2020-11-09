use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

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
Given the root of a binary tree, find the maximum value V for which there exist different nodes A
and B where V = |A.val - B.val| and A is an ancestor of B.

A node A is an ancestor of B if either: any child of A is equal to B, or any child of A is an
ancestor of B.
*/

fn helper(node: &Tree, mut max_ancestor: i32, mut min_ancestor: i32) -> i32 {
    if let Some(inner) = node {
        let inner = inner.borrow();
        max_ancestor = cmp::max(max_ancestor, inner.val);
        min_ancestor = cmp::min(min_ancestor, inner.val);
        return cmp::max((inner.val - max_ancestor).abs(),
            cmp::max((inner.val - min_ancestor).abs(), 
                cmp::max(helper(&inner.left, max_ancestor, min_ancestor),
                    helper(&inner.right, max_ancestor, min_ancestor))));
    }
    0
}

pub fn max_ancestor_diff(root: Tree) -> i32 {
    if let Some(inner) = root {
        let inner = inner.borrow();
        return cmp::max(helper(&inner.left, inner.val, inner.val),
            helper(&inner.right, inner.val, inner.val));
    }
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
