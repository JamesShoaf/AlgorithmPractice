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

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
    if let Some(inner) = node {
        let inner = inner.borrow();
        if inner.left.is_none() && inner.right.is_none() {
            if is_left { return inner.val; }
            return 0;
        }
        return dfs(&inner.left, true) + dfs(&inner.right, false)
    }
    0
}

fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    dfs(&root, false)
}
