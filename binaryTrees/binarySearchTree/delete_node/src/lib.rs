use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

fn helper(root: &mut Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    type T = Option<Rc<RefCell<TreeNode>>>;
    let mut left: *mut T = &mut None as *mut T;
    let mut right: *mut T = &mut None as *mut T;
    if let Some(rc) = root {
        let mut ref_mut = rc.borrow_mut();
        if ref_mut.val > key {
            println!("key has lesser value than node value: {}", ref_mut.val);
            let mut next_left = helper(&mut ref_mut.left, key);
            if let Some(inner_left) = next_left.take() {
                ref_mut.left.replace(inner_left);
            } else { ref_mut.left.take(); }
        }
        if ref_mut.val < key {
            println!("key has greater value than node value: {}", ref_mut.val);
            let mut next_right = helper(&mut ref_mut.right, key);
            if let Some(inner_right) = next_right.take() {
                ref_mut.right.replace(inner_right);
            } else { ref_mut.right.take(); }
        }
        if ref_mut.val == key {
            println!("key has equal value to node value: {}", ref_mut.val);
            left = &mut ref_mut.left as *mut T;
            right = &mut ref_mut.right as *mut T;
        }
    }
    unsafe {
        if let Some(inner_right) = &*right {
            root.replace(inner_right.clone());
            if let Some(inner_left) = &*left {
                let mut temp = right;
                loop {
                    if let Some(inner_temp) = &*temp {
                        let mut ref_mut = inner_temp.borrow_mut();
                        if ref_mut.left.is_none() { break; }
                        temp = &mut ref_mut.left as *mut T;
                    }    
                }
                let temp_node = &mut *temp;
                temp_node.replace(inner_left.clone());
            }
        } else if let Some(inner_left) = &*left {
            root.replace(inner_left.clone());
        } else {
            root.take();
        }
    }
    if let Some(inner) = root {
        return Some(inner.clone());
    }
    else { return None; }
}

fn delete_node(mut root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    helper(&mut root, key)
}

#[test]
fn it_works() {
    let mut root = TreeNode::new(5);
    let mut l = TreeNode::new(3);
    let ll = TreeNode::new(3);
    let lr = TreeNode::new(3);
    l.left = Some(Rc::new(RefCell::new(ll)));
    l.right = Some(Rc::new(RefCell::new(lr)));
    root.left = Some(Rc::new(RefCell::new(l)));
    let mut r = TreeNode::new(3);
    let rr = TreeNode::new(3);
    r.right = Some(Rc::new(RefCell::new(rr)));
    root.right = Some(Rc::new(RefCell::new(r)));

    delete_node(Some(Rc::new(RefCell::new(root))), 3);
}