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

type Node = Option<Rc<RefCell<TreeNode>>>;

fn dfs(node: &Node) -> i32 {
    if let Some(rc) = node {
        let inner = rc.borrow();
        if inner.left.is_some() && inner.right.is_some() {
            return 1 + cmp::min(dfs(&inner.left), dfs(&inner.right));
        }
        return 1 + cmp::max(dfs(&inner.left), dfs(&inner.right));
    }
    0
}

pub fn min_depth(root: Node) -> i32 {
    dfs(&root)
}

/* 
#1 solution for comparison

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(r) => {
            use std::collections::VecDeque;
            let mut deque: VecDeque<(Option<Rc<RefCell<TreeNode>>>, i32)> = VecDeque::new();
            let mut min_dep = 0;

            deque.push_back((Some(r), 1));
            while let Some((Some(node), height)) = deque.pop_front() {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    min_dep =  height;
                    break;
                } else {
                    if node.borrow().left.is_some() {
                        deque.push_back((node.borrow().left.clone(), height + 1));
                    }
                    if node.borrow().right.is_some() {
                        deque.push_back((node.borrow().right.clone(), height + 1));
                    }
                }
            }
            min_dep
        }
    }
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
