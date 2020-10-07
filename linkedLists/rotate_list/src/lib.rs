#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut list: Option<Box<ListNode>> = None;
    for num in vec.into_iter().rev() {
        let mut temp = ListNode::new(num);
        temp.next = list;
        list = Some(Box::new(temp));
    }
    list
}

fn to_vec(list: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    if let Some(node) = list {
        vec.push(node.val);
        vec.append(&mut to_vec(&node.next));
    }
    vec
}

// Given a linked list, rotate the list to the right by k places, where k is non-negative.
pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let vec = to_vec(&head);
    let l = vec.len();
    if l < 2 || k as usize % l == 0 {
        return from_vec(vec);
    }
    let k = k as usize % l;
    from_vec([&vec[l - k..l], &vec[0..l - k]].concat())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_vec() {
        assert_eq!(to_vec(&None), Vec::new());
        let list1 = Some(Box::new(ListNode::new(1)));
        assert_eq!(to_vec(&list1), vec![1]);
        let list2b = Some(Box::new(ListNode::new(3)));
        let mut list2a = ListNode::new(2);
        list2a.next = list2b;
        let list2 = Some(Box::new(list2a));
        assert_eq!(to_vec(&list2), vec![2, 3]);
    }

    #[test]
    fn test_from_vec() {
        let test_vecs = vec![
            Vec::new(),
            vec![1],
            vec![2, 3],
            vec![3, 4, 5, 6, 7],
        ];
        for vec in test_vecs {
            let output = to_vec(&from_vec(vec.clone()));
            assert_eq!(output, vec);
        }
    }

    #[test]
    fn test_rotate_right() {
        let test_tuples = vec![
            (Vec::new(), 1, Vec::new()),
            (vec![1], 1, vec![1]),
            (vec![1], 10, vec![1]),
            (vec![1, 2], 0, vec![1, 2]),
            (vec![1, 2], 1, vec![2, 1]),
            (vec![1, 2, 3, 4, 5], 0, vec![1, 2, 3, 4, 5]),
            (vec![1, 2, 3, 4, 5], 5, vec![1, 2, 3, 4, 5]),
            (vec![1, 2, 3, 4, 5], 1, vec![5, 1, 2, 3, 4]),
            (vec![1, 2, 3, 4, 5], 2, vec![4, 5, 1, 2, 3]),
            (vec![1, 2, 3, 4, 5], 7, vec![4, 5, 1, 2, 3]),
            (vec![1, 2, 3, 4, 5], 3, vec![3, 4, 5, 1, 2]),
            (vec![1, 2, 3, 4, 5], 4, vec![2, 3, 4, 5, 1]),
        ];
        for (init, k, expected) in test_tuples {
            assert_eq!(to_vec(&rotate_right(from_vec(init), k)), expected);
        }
    }
}

/* 
#1 solution for comparison

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut vals = Vec::new();

    let mut node = head.clone();
    while let Some(n) = node {
        vals.push(n.val);
        node = n.next;
    }

    if vals.len() == 1 {
        return Some(Box::new(ListNode::new(vals[0])))

    }

    let i = vals.len() - (k as usize % vals.len());
    let mut node = None;
    for v in vals[..i].iter().rev().chain(vals[i..].iter().rev()) {
        let tmp = node;
        let mut new_node = ListNode::new(*v);
        new_node.next = tmp;
        node = Some(Box::new(new_node));
    }
    node
}
*/