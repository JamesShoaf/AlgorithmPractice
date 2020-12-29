/*
Given a binary tree where node values are digits from 1 to 9. A path in the binary tree is said to
be pseudo-palindromic if at least one permutation of the node values in the path is a palindrome.

Return the number of pseudo-palindromic paths going from the root node to leaf nodes.
*/

use serialize_deserialize::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

fn recursive_helper(node: &Tree, parity_mask: &mut u32) -> i32 {
    if let Some(inner) = node {
        let inner = inner.borrow();
        *parity_mask ^= 1 << inner.val;
        let res = if inner.left.is_none() && inner.right.is_none() {
            (parity_mask.count_ones() <= 1) as i32
        } else {
            recursive_helper(&inner.left, parity_mask) + recursive_helper(&inner.right, parity_mask)
        };
        *parity_mask ^= 1 << inner.val;
        return res;
    }
    0
}

pub fn pseudo_palindromic_paths(root: Tree) -> i32 {
    recursive_helper(&root, &mut 0)
}

#[cfg(test)]
mod tests {
    use super::pseudo_palindromic_paths;
    use serialize_deserialize::deserialize;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            ("".to_string(), 0),
            ("1".to_string(), 1),
            ("1,1".to_string(), 1),
            ("1,1,1".to_string(), 2),
            ("2,3,1,3,1,x,1".to_string(), 2),
            ("2,1,1,1,3,x,x,x,x,x,1".to_string(), 1),
        ];
        for (serial, expected) in test_tuples {
            assert_eq!(pseudo_palindromic_paths(deserialize(serial)), expected);
        }
    }
}
