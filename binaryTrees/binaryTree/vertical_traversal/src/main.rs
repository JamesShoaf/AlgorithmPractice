/* 
Given a binary tree, return the vertical order traversal of its nodes values.

For each node at position (X, Y), its left and right children respectively will be at positions
(X-1, Y-1) and (X+1, Y-1).

Running a vertical line from X = -infinity to X = +infinity, whenever the vertical line touches some
nodes, we report the values of the nodes in order from top to bottom (decreasing Y coordinates).

If two nodes have the same position, then the value of the node that is reported first is the value
that is smaller.

Return an list of non-empty reports in order of X coordinate.  Every report will have a list of
values of nodes.
*/

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

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() { return Vec::new(); }
        // node, node's x value
        let mut queue: Vec<(Rc<RefCell<TreeNode>>, i32)> = Vec::new();
        let mut min_x = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        // x, y, vals
        let mut x_map: HashMap<i32, HashMap<i32, Vec<i32>>> = HashMap::new();
        queue.push((root.unwrap(), 0));
        loop {
            let mut next_queue = Vec::new();
            for (rc, x) in &queue {
                if *x < min_x { min_x = *x; }
                if *x > max_x { max_x = *x; }
                let current = rc.borrow();
                if let Some(right) = &current.right { next_queue.push((right.clone(), x + 1));}
                if let Some(left) = &current.left { next_queue.push((left.clone(), x - 1));}
                x_map.entry(*x).or_insert(HashMap::new()).entry(max_y).or_insert(Vec::new()).push(current.val);
            }
            if next_queue.is_empty() { break; }
            queue = next_queue;
            max_y += 1;
        }
        let mut output: Vec<Vec<i32>> = Vec::new();
        for x in min_x..=max_x {
            if let Some(y_map) = x_map.get(&x) {
                let mut vert = Vec::new();
                for y in 0..=max_y {
                    if let Some(vec) = y_map.get(&y) {
                        let mut coord = vec.clone();
                        coord.sort();
                        vert.append(&mut coord);
                    }
                }
                output.push(vert);
            }
        }
        output
    }
}

fn main() {
    let mut root = TreeNode::new(1);
    let mut left = TreeNode::new(2);
    let left_left = TreeNode::new(4);
    let left_right = TreeNode::new(6);
    left.left = Some(Rc::new(RefCell::new(left_left)));
    left.right = Some(Rc::new(RefCell::new(left_right)));
    root.left = Some(Rc::new(RefCell::new(left)));
    let mut right = TreeNode::new(3);
    let right_left = TreeNode::new(5);
    let right_right = TreeNode::new(7);
    right.left = Some(Rc::new(RefCell::new(right_left)));
    right.right = Some(Rc::new(RefCell::new(right_right)));
    root.right = Some(Rc::new(RefCell::new(right)));
    println!("{:?}", Solution::vertical_traversal(Some(Rc::new(RefCell::new(root)))));
}
