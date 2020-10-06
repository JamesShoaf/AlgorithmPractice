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

fn helper(node: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
    if node.is_none() {
        node.replace(Rc::new(RefCell::new(TreeNode::new(val))));
    }
    if let Some(rc) = node {
        let mut ref_mut = rc.borrow_mut();
        if val > ref_mut.val {
            helper(&mut ref_mut.right, val);
        }
        if val < ref_mut.val {
            helper(&mut ref_mut.left, val);
        }
    }
}

pub fn insert(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    helper(&mut root, val);
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut root = TreeNode::new(5);
        let mut l = TreeNode::new(3);
        let ll = TreeNode::new(2);
        let lr = TreeNode::new(4);
        l.left = Some(Rc::new(RefCell::new(ll)));
        l.right = Some(Rc::new(RefCell::new(lr)));
        root.left = Some(Rc::new(RefCell::new(l)));
        let mut r = TreeNode::new(7);
        let rr = TreeNode::new(8);
        r.right = Some(Rc::new(RefCell::new(rr)));
        root.right = Some(Rc::new(RefCell::new(r)));

        println!("{:?}", insert(Some(Rc::new(RefCell::new(root))), 6));
        panic!();
    }
}
