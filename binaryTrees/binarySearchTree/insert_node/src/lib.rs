use std::rc::Rc;
use std::cell::RefCell;

// alias length type
type Tree = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Tree,
  pub right: Tree,
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

fn insert(tree: &mut Tree, val: i32) {
    // get a mutable reference to the current TreeNode
    if let Some(rc) = tree {
        let mut ref_mut = rc.borrow_mut();
        // then recurse on the right or left branch until an empty leaf is found
        if val > ref_mut.val {
            insert(&mut ref_mut.right, val);
        }
        if val < ref_mut.val {
            insert(&mut ref_mut.left, val);
        }
    // if the existing tree is empty or when insert recurses to an empty leaf, insert the value
    } else {
        tree.replace(Rc::new(RefCell::new(TreeNode::new(val))));
    }
}

pub fn owned_insert(mut root: Tree, val: i32) -> Tree {
    insert(&mut root, val);
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

        println!("{:?}", owned_insert(Some(Rc::new(RefCell::new(root))), 6));
        panic!();
    }
}
