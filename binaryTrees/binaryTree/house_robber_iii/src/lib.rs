/*
The thief has found himself a new place for his thievery again. There is only one entrance to this
area, called the "root." Besides the root, each house has one and only one parent house. After a
tour, the smart thief realized that "all houses in this place forms a binary tree". It will
automatically contact the police if two directly-linked houses were broken into on the same night.

Determine the maximum amount of money the thief can rob tonight without alerting the police.
*/

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

type Ptr = Rc<RefCell<TreeNode>>;
type Tree = Option<Ptr>;

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

// (best_val_w_or_wo_node, best_val_wo_node)
fn dfs(node: &Tree) -> (i32, i32) {
    if let Some(inner) = node {
        let inner = inner.borrow();
        let left = dfs(&inner.left);
        let right = dfs(&inner.right);
        (
            cmp::max(inner.val + left.1 + right.1, left.0 + right.0),
            left.0 + right.0,
        )
    } else {
        (0, 0)
    }
}

pub fn rob(root: Tree) -> i32 {
    dfs(&root).0
}

// initial implementation

// fn get_granchildren(node: Ptr) -> Vec<Ptr> {
//     let mut res: Vec<Ptr> = Vec::new();
//     let node = node.borrow();
//     if let Some(left) = &node.left {
//         let left = left.borrow();
//         if let Some(left_left) = &left.left {
//             res.push(Rc::clone(left_left));
//         }
//         if let Some(left_right) = &left.right {
//             res.push(Rc::clone(left_right));
//         }
//     }
//     if let Some(right) = &node.right {
//         let right = right.borrow();
//         if let Some(right_left) = &right.left {
//             res.push(Rc::clone(right_left));
//         }
//         if let Some(right_right) = &right.right {
//             res.push(Rc::clone(right_right));
//         }
//     }
//     res
// }

// fn recursive_rob(node: Ptr) -> i32 {
//     let grandchildren = get_granchildren(Rc::clone(&node));
//     let node = node.borrow();
//     let mut sum_with_current = node.val;
//     for grandchild in grandchildren {
//         sum_with_current += recursive_rob(grandchild);
//     }
//     let mut sum_without_current = 0;
//     if let Some(left) = &node.left {
//         sum_without_current += recursive_rob(Rc::clone(left));
//     }
//     if let Some(right) = &node.right {
//         sum_without_current += recursive_rob(Rc::clone(right));
//     }
//     cmp::max(sum_with_current, sum_without_current)
// }

// pub fn rob(root: Tree) -> i32 {
//     if let Some(inner) = &root {
//         recursive_rob(Rc::clone(inner))
//     } else {
//         0
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
