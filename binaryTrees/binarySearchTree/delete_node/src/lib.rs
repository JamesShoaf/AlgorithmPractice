use std::rc::Rc;
use std::cell::RefCell;
use serialize_deserialize::TreeNode;

type Tree = Option<Rc<RefCell<TreeNode>>>;

// recursively traverses left in the right tree to the first empty node, then inserts the content
// from the left tree
fn inject_left(right: &mut Tree, left: &mut Tree) {
    if let Some(rc) = right {
        return inject_left(&mut rc.borrow_mut().left, left);
    }
    if left.is_some() {
        right.replace(left.take().unwrap());
    }
}

// replace the current node with its children's combined contents
fn rotate_branches(tree: &mut Tree) {
    if tree.is_none() { return; }
    let mut left = tree.as_ref().unwrap().borrow_mut().left.take();
    let mut right = tree.as_ref().unwrap().borrow_mut().right.take();
    tree.take();
    inject_left(&mut right, &mut left);
    if let Some(inner) = right {
        tree.replace(inner);
    }
}

// given a binary search tree, remove val from the tree, if it exists
pub fn delete(tree: &mut Tree, val: i32) {
    if let Some(rc) = tree {
        let mut ref_mut = rc.borrow_mut();
        if val < ref_mut.val {
            return delete(&mut ref_mut.left, val);
        }
        if val > ref_mut.val {
            return delete(&mut ref_mut.right, val);
        }
    }
    rotate_branches(tree);
}

pub fn owned_delete(mut root: Tree, key: i32) -> Tree {
    delete(&mut root, key);
    root
}


#[test]
fn test_delete() {
    use serialize_deserialize::{ serialize, deserialize };
    let test_tuples = vec![
        (String::new(), 1, String::new()),
        (String::from("1"), 1, String::from("")),
        (String::from("2,1"), 1, String::from("2")),
        (String::from("1,x,2"), 2, String::from("1")),
        (String::from("3,2,x,1"), 1, String::from("3,2")),
        (String::from("1,x,2,x,3"), 3, String::from("1,x,2")),
        (String::from("5,3,7,2,4,x,8"), 5, String::from("7,3,8,2,4")),
        (String::from("5,3,7,2,4,x,8"), 3, String::from("5,4,7,2,x,x,8")),
        (String::from("5,3,7,2,4,x,8"), 7, String::from("5,3,8,2,4")),
        (String::from("5,3,7,2,4,x,8"), 2, String::from("5,3,7,x,4,x,8")),
        (String::from("5,3,7,2,4,x,8"), 4, String::from("5,3,7,2,x,x,8")),
        (String::from("5,3,7,2,4,x,8"), 8, String::from("5,3,7,2,4")),
    ];
    for (serial, val, expected) in test_tuples {
        let mut tree = deserialize(serial);
        delete(&mut tree, val);
        assert_eq!(serialize(&tree), expected);
    }
}