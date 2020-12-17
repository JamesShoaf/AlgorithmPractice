use serialize_deserialize::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

fn recursive_helper(node: &Tree, min_valid: Option<i32>, max_valid: Option<i32>) -> bool {
    if let Some(inner) = node {
        let inner = inner.borrow();
        if let Some(min) = min_valid {
            if inner.val < min {
                return false;
            }
        } else {
            return false;
        }
        if let Some(max) = max_valid {
            if inner.val > max {
                return false;
            }
        } else {
            return false;
        }
        return recursive_helper(&inner.left, min_valid, inner.val.checked_sub(1))
            && recursive_helper(&inner.right, inner.val.checked_add(1), max_valid);
    }
    true
}

pub fn is_valid_bst(root: Tree) -> bool {
    recursive_helper(&root, Some(i32::MIN), Some(i32::MAX))
}

/*
#1 solution for comparison
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut nodes: Vec<(Rc<RefCell<TreeNode>>, Option<i32>, Option<i32>)>  = vec![(Rc::clone(&root.unwrap()), None, None)];
        while !nodes.is_empty() {
            let (node, upper, lower): (Rc<RefCell<TreeNode>>, Option<i32>, Option<i32>) = nodes.pop().unwrap();
            let node: &TreeNode = &node.as_ref().borrow();
            let node_val = node.val;
            if let Some(left) = &node.left {
                match Solution::in_left_bound(left, node_val, lower) {
                    true => nodes.push((
                        Rc::clone(left),
                        Some(node_val),
                        lower
                    )),
                    false => return false,
                }
            }
            if let Some(right) = &node.right {
                match Solution::in_right_bound(right, node_val, upper) {
                    true => nodes.push((
                        Rc::clone(right),
                        upper,
                        Some(node_val)
                    )),
                    false => return false,
                }
            }
        }
        true
    }
    pub fn in_left_bound(left: &Rc<RefCell<TreeNode>>, node_val:i32, lower: Option<i32>) -> bool {
        let left_val = left.as_ref().borrow().val;
        (lower.is_none() || lower.unwrap() < left_val) && (left_val < node_val)
    }
    pub fn in_right_bound(right: &Rc<RefCell<TreeNode>>, node_val:i32, upper: Option<i32>) -> bool {
        let right_val = right.as_ref().borrow().val;
        (upper.is_none() || upper.unwrap() > right_val) && (right_val > node_val)
    }
}
*/

#[cfg(test)]
mod tests {
    use super::is_valid_bst;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from(""), true),
            (format!("{}", i32::MAX), true),
            (format!("{}", i32::MIN), true),
            (format!("{},{}", i32::MIN, i32::MAX), false),
            (format!("{},{}", i32::MAX, i32::MIN), true),
            (format!("{},x,{}", i32::MIN, i32::MAX), true),
            (format!("{},x,{}", i32::MAX, i32::MIN), false),
            (String::from("5,1,4,null,null,3,6"), false),
            (String::from("2,1,3"), true),
        ];
        for (serial, expected) in test_tuples {
            assert_eq!(
                is_valid_bst(serialize_deserialize::deserialize(serial)),
                expected
            );
        }
    }
}
