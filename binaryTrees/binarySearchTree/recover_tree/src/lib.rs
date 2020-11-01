use serialize_deserialize::*;
use std::rc::Rc;
use std::cell::RefCell;

type Tree = Option<Rc<RefCell<TreeNode>>>;

fn find_node_by_val(val: i32, root: &Tree) -> Tree {
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    if let Some(node) = root {
        stack.push(Rc::clone(&node));
    }
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        if node.borrow().val == val {
            return Some(node);
        }
        if node.borrow().left.is_some() {
            stack.push(Rc::clone(node.borrow().left.as_ref().unwrap()))
        }
        if node.borrow().right.is_some() {
            stack.push(Rc::clone(node.borrow().right.as_ref().unwrap()))
        }
    }

    None
}

fn swap_val(a: i32, b: i32, root: &Tree) {
    let a = find_node_by_val(a, root);
    let b = find_node_by_val(b, root);
    if a.is_none() || b.is_none() {return;}
    std::mem::swap(&mut a.unwrap().borrow_mut().val, &mut b.unwrap().borrow_mut().val);
}

// type TreeRc = Rc<RefCell<TreeNode>>;

// pub fn recover_tree(root: &mut Tree) {
//     let mut current = Some(Rc::clone(root.as_ref().unwrap()));
//     let mut stack: Vec<TreeRc> = Vec::new();
//     let mut first_pair: Option<(TreeRc, TreeRc)> = None;
//     let mut second_pair: Option<(TreeRc, TreeRc)> = None;
//     let mut temp_a: Option<i32> = None;
//     let mut temp_b: Option<i32> = None;
//     while current.is_some() {
//         let rc = current.as_ref().unwrap();
//         stack.push(Rc::clone(rc));
//         if let Some(left) = &rc.borrow().left {
//             current = Some(Rc::clone(left));
//         }
//     }
// }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
