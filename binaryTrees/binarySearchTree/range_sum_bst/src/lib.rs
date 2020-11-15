/*
Given the root node of a binary search tree, return the sum of values of all nodes with a value in the range [low, high].
*/

use serialize_deserialize::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

//recursive
fn recursive_helper(node: &Tree, low: &i32, high: &i32, res: &mut i32) {
    if let Some(inner) = node {
        let inner = inner.borrow();
        if inner.val > *low {
            recursive_helper(&inner.left, low, high, res);
        }
        if inner.val >= *low && inner.val <= *high {
            *res += inner.val;
        }
        if inner.val < *high {
            recursive_helper(&inner.right, low, high, res);
        }
    }
}

pub fn range_sum_bst(root: Tree, low: i32, high: i32) -> i32 {
    let mut res = 0;
    recursive_helper(&root, &low, &high, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use serialize_deserialize::deserialize;
    #[test]
    fn recursive_range_sum_bst() {
        let test_tuples = vec![
            (String::from(""), -1, 1, 0),
            (String::from("5"), -1, 1, 0),
            (String::from("10,5,15,3,7,x,18"), 7, 15, 32),
            (String::from("10,5,15,3,7,13,18,1,x,6"), 6, 10, 23),
        ];
        for (serial, low, high, expected) in test_tuples {
            let root = deserialize(serial);
            assert_eq!(range_sum_bst(root, low, high), expected);
        }
    }
}
