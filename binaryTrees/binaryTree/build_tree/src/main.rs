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

struct Solution {}

// given inorder and postorder traversal of a binary tree, construct the binary tree
// assume no duplicate values exist in the tree
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(()).filter(|_| postorder.len() > 0)?;
        let mut inorder_left = inorder.to_vec();
        let mut postorder_left = postorder.to_vec();
        let root_val = postorder_left.pop().unwrap();
        let mut root_node = TreeNode::new(root_val);
        let inorder_root_index = inorder_left.iter().position(|&x| x == root_val).unwrap();
        let inorder_right = inorder_left.split_off(inorder_root_index + 1);
        inorder_left.pop();
        let postorder_right = postorder_left.split_off(inorder_left.len());
        root_node.left = Solution::build_tree(inorder_left, postorder_left);
        root_node.right = Solution::build_tree(inorder_right, postorder_right);
        Some(Rc::new(RefCell::new(root_node)))
    }
}

fn main() {
    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];
    let output = Solution::build_tree(inorder.clone(), postorder.clone());
    println!("{:#?}", output);
}
