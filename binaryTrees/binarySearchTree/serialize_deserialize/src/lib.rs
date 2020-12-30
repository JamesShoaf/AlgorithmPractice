use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
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

pub fn is_equal(node1: &Tree, node2: &Tree) -> bool {
    if node1.is_none() && node2.is_none() {
        return true;
    }
    if node1.is_none() || node2.is_none() {
        return false;
    }
    let inner1 = node1.as_ref().unwrap().borrow();
    let inner2 = node2.as_ref().unwrap().borrow();
    if inner1.val != inner2.val {
        return false;
    }
    is_equal(&inner1.left, &inner2.left) && is_equal(&inner1.right, &inner2.right)
}

fn clone_contents(node: &Tree) -> Tree {
    if let Some(inner) = node.as_ref() {
        return Some(inner.clone());
    }
    None
}

fn push_char(queue: &mut VecDeque<Tree>, output: &mut String) {
    let current = queue.pop_front().unwrap();
    if let Some(inner) = current {
        let inner = inner.borrow();
        *output += &inner.val.to_string();
        queue.push_back(clone_contents(&inner.left));
        queue.push_back(clone_contents(&inner.right));
    } else {
        output.push('x');
    }
}

pub fn serialize(root: &Tree) -> String {
    if root.is_none() {
        return String::new();
    }
    let mut output = String::new();
    let mut queue = VecDeque::from(vec![Some(root.as_ref().unwrap().clone())]);
    push_char(&mut queue, &mut output);
    while queue.iter().any(|x| x.is_some()) {
        output.push(',');
        push_char(&mut queue, &mut output);
    }
    output
}

fn add_node(
    node: &mut Tree,
    nodes: &mut std::str::Split<char>,
    queue: &mut VecDeque<Rc<RefCell<TreeNode>>>,
) {
    if let Some(inner) = nodes.next() {
        if let Ok(inner_val) = inner.parse() {
            let left_node = Rc::new(RefCell::new(TreeNode::new(inner_val)));
            queue.push_back(left_node.clone());
            node.replace(left_node);
        }
    }
}

pub fn deserialize(s: String) -> Tree {
    Some(()).filter(|_| s.len() > 0)?;
    let mut nodes = s.split(',');
    let root = Rc::new(RefCell::new(TreeNode::new(
        nodes.next().unwrap().parse().unwrap(),
    )));
    let mut queue = VecDeque::from(vec![root.clone()]);
    while !queue.is_empty() {
        let current = queue.pop_front();
        let mut current = current.as_ref().unwrap().borrow_mut();
        add_node(&mut current.left, &mut nodes, &mut queue);
        add_node(&mut current.right, &mut nodes, &mut queue);
    }
    Some(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup_test_trees() -> Vec<Tree> {
        let mut test_trees = Vec::new();

        test_trees.push(None);
        test_trees.push(Some(Rc::new(RefCell::new(TreeNode::new(1)))));

        let mut two_node = TreeNode::new(1);
        two_node.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        test_trees.push(Some(Rc::new(RefCell::new(two_node))));

        let mut three_node = TreeNode::new(1);
        let mut three_child = TreeNode::new(2);
        three_child.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        three_node.right = Some(Rc::new(RefCell::new(three_child)));
        test_trees.push(Some(Rc::new(RefCell::new(three_node))));

        let l_r_l = TreeNode::new(2);
        let mut l_r = TreeNode::new(3);
        l_r.left = Some(Rc::new(RefCell::new(l_r_l)));
        let mut l = TreeNode::new(1);
        l.right = Some(Rc::new(RefCell::new(l_r)));
        let r = TreeNode::new(5);
        let mut four_node = TreeNode::new(4);
        four_node.left = Some(Rc::new(RefCell::new(l)));
        four_node.right = Some(Rc::new(RefCell::new(r)));
        test_trees.push(Some(Rc::new(RefCell::new(four_node))));

        test_trees
    }

    #[test]
    fn test_is_equal() {
        let test_trees = setup_test_trees();
        for i in 0..test_trees.len() {
            assert!(test_trees[i] == test_trees[i]);
            assert!(is_equal(&test_trees[i], &test_trees[i]));
            for j in i + 1..test_trees.len() {
                assert!(test_trees[i] != test_trees[j]);
                assert!(!is_equal(&test_trees[i], &test_trees[j]));
            }
        }
    }

    #[test]
    fn test_serialize() {
        let test_trees = setup_test_trees();
        let expected = vec![
            String::new(),
            String::from("1"),
            String::from("1,x,2"),
            String::from("1,x,2,x,3"),
            String::from("4,1,5,x,3,x,x,2"),
        ];
        for (tree, s) in test_trees.into_iter().zip(expected.into_iter()) {
            assert_eq!(serialize(&tree), s);
        }
    }

    #[test]
    fn test_deserialize() {
        let test_strings = vec![
            String::from(""),
            String::from("1"),
            String::from("2,1"),
            String::from("4,1,5,x,3,x,x,2"),
        ];
        for s in test_strings {
            assert_eq!(serialize(&deserialize(s.clone())), s);
        }
        for tree in setup_test_trees() {
            assert_eq!(deserialize(serialize(&tree)), tree);
        }
    }
}
