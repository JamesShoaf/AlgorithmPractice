use std::cell::RefCell;
use std::rc::Rc;

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
            right: None,
        }
    }
}

type TreeRc = Rc<RefCell<TreeNode>>;
type Tree = Option<TreeRc>;

fn recursive_helper(node: &Tree, depth: usize) -> Option<(TreeRc, usize)> {
    if let Some(rc) = node {
        let inner = rc.borrow();
        if let Some((l_rc, l_depth)) = recursive_helper(&inner.left, depth + 1) {
            if let Some((r_rc, r_depth)) = recursive_helper(&inner.right, depth + 1) {
                // if the right child is deeper select it
                if r_depth > l_depth {
                    return Some((r_rc, r_depth));
                }
                // if left and right have the same depth, return the root
                if r_depth == l_depth {
                    return Some((Rc::clone(rc), l_depth));
                }
            }
            // if left is deeper or there is no right
            return Some((l_rc, l_depth));
        }
        // if there's a right child, select it
        if let Some(right) = recursive_helper(&inner.right, depth + 1) {
            return Some(right);
        }
        // if there's no children, return the root
        return Some((Rc::clone(rc), depth));
    }
    None
}

pub fn subtree_with_all_deepest(root: Tree) -> Tree {
    if let Some((rc, _)) = recursive_helper(&root, 0) {
        return Some(rc);
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
