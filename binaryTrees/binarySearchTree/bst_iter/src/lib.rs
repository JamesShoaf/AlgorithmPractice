use serialize_deserialize::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
type TreeRc = Rc<RefCell<TreeNode>>;
type Tree = Option<TreeRc>;

pub struct BSTIterator {
    stack: Vec<TreeRc>,
}

impl BSTIterator {
    fn push_left(rc: &TreeRc, stack: &mut Vec<TreeRc>) {
        let mut current = Some(Rc::clone(rc));
        while let Some(curr_rc) = current {
            stack.push(Rc::clone(&curr_rc));
            current = if let Some(left) = &curr_rc.borrow().left {
                Some(Rc::clone(left))
            } else {
                None
            };
        }
    }
    pub fn new(root: Tree) -> Self {
        let mut stack: Vec<TreeRc> = Vec::new();
        if let Some(root_rc) = &root {
            BSTIterator::push_left(root_rc, &mut stack);
        }
        BSTIterator { stack }
    }
    pub fn next(&mut self) -> i32 {
        if let Some(top) = self.stack.pop() {
            let val = top.borrow().val;
            if let Some(right) = &top.borrow().right {
                BSTIterator::push_left(right, &mut self.stack);
            }
            return val;
        }
        -1
    }
    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serialize_deserialize::deserialize;
    #[test]
    fn it_works() {
        let test_serials = vec![
            String::from(""),
            String::from("1,x,2"),
            String::from("2,1,3,0,x,x,4"),
        ];
        for s in test_serials {
            let mut vals: Vec<i32> = Vec::new();
            for val in s.split(',').filter(|&val| val != "x") {
                if let Ok(val) = val.parse() {
                    vals.push(val)
                }
            }
            vals.sort();
            let mut bst_iter = BSTIterator::new(deserialize(s));
            for val in vals {
                assert!(bst_iter.has_next());
                assert_eq!(bst_iter.next(), val);
            }
            assert!(!bst_iter.has_next());
            assert_eq!(bst_iter.next(), -1);
        }
    }
}
