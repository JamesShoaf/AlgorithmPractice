use std::rc::Rc;
use std::cell::RefCell;

struct TreeNode {
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

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut output: Vec<Vec<i32>> = Vec::new();
        let mut queue: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut reverse_order: bool = false;
        if let Some(rc) = root {
            queue.push(rc);
        }
        loop {
            if queue.len() == 0 { break; }
            let mut next_level: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
            let mut values: Vec<i32> = Vec::new();
            for rc in queue.iter() {
                let tree = rc.borrow();
                values.push(tree.val);
                let mut children: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
                if let Some(left) = &tree.left { children.push(left.clone()); }
                if let Some(right) = &tree.right { children.push(right.clone()); }
                if reverse_order { children.reverse(); }
                next_level.append(&mut children);
            }
            output.push(values);
            queue = next_level;
            queue.reverse();
            reverse_order = !reverse_order;
        }
        
        output
    }
}

fn main() {
    let mut one = TreeNode::new(1);
    let mut two = TreeNode::new(2);
    let mut three = TreeNode::new(3);
    let four = TreeNode::new(4);
    let five = TreeNode::new(5);
    let six = TreeNode::new(6);
    three.left = Some(Rc::new(RefCell::new(five)));
    three.right = Some(Rc::new(RefCell::new(six)));
    two.left = Some(Rc::new(RefCell::new(four)));
    one.right = Some(Rc::new(RefCell::new(three)));
    one.left = Some(Rc::new(RefCell::new(two)));

    let zigzag = Solution::zigzag_level_order(Some(Rc::new(RefCell::new(one))));
    println!("{:#?}", zigzag);
}
