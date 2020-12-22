use serialize_deserialize::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

fn is_branch_balanced(node: &Tree) -> Option<i32> {
    if let Some(inner) = node {
        let inner = inner.borrow();
        if let Some(left_height) = is_branch_balanced(&inner.left) {
            if let Some(right_height) = is_branch_balanced(&inner.right) {
                if (left_height - right_height).abs() <= 1 {
                    return Some(cmp::max(left_height, right_height) + 1);
                }
            }
        }
        return None;
    }
    Some(0)
}

pub fn is_balanced(root: Tree) -> bool {
    is_branch_balanced(&root).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from(""), true),
            (String::from("10,5,15,3,8,x,20,1"), true),
            (String::from("3,9,20,x,x,15,7"), true),
            (String::from("1,2,x,3"), false),
            (String::from("1,2,2,3,3,x,x,4,4"), false),
        ];
        for (serial, expected) in test_tuples {
            let tree = serialize_deserialize::deserialize(serial);
            assert_eq!(is_balanced(tree), expected);
        }
    }
}
