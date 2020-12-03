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

use std::cell::RefCell;
use std::rc::Rc;
type TreeRc = Rc<RefCell<TreeNode>>;
type Tree = Option<TreeRc>;

// traverse rightward along a linear tree to the tail, then appends contents
fn insert_tail(node: &mut Tree, contents: TreeRc) {
    if let Some(node) = node {
        let mut node = node.borrow_mut();
        return insert_tail(&mut node.right, contents);
    }
    node.replace(contents);
}

// rearrange the left, insert the current node at the tail of the left, then recurse on the right
fn recursive_rearrange(node: &mut Tree) {
    if let Some(inner) = node.take() {
        recursive_rearrange(&mut inner.borrow_mut().left);
        if let Some(left) = inner.borrow_mut().left.take() {
            node.replace(left);
        }
        insert_tail(node, Rc::clone(&inner));
    }
    if let Some(inner) = node {
        recursive_rearrange(&mut inner.borrow_mut().right)
    }
}

pub fn increasing_bst(mut root: Tree) -> Tree {
    recursive_rearrange(&mut root);
    root
}

/*
#1 solution for comparison

// 中序遍历，每次访问的节点作为上一次访问节点的right
pub fn increasing_bst(
    mut root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut new_root = None;
    let mut parent: Option<Rc<RefCell<TreeNode>>> = None;
    let mut stack = Vec::with_capacity(32);
    while !stack.is_empty() || root.is_some() {
        while root.is_some() {
            let node = root.unwrap();
            root = node.borrow_mut().left.take();
            stack.push(node);
        }
        root = stack.pop();
        if new_root.is_none() {
            // 第一次遍历到最左的叶子节点，这个节点的值是最小的，作为根节点。
            new_root = root.clone();
        }
        if let Some(p) = parent {
            p.borrow_mut().right = root.clone();
        }
        parent = root.clone();
        let node = root.unwrap();
        root = node.borrow_mut().right.take();
    }
    new_root
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
