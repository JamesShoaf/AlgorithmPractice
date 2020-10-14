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

type Node = Option<Box<ListNode>>;

fn from_vec(vec: Vec<i32>) -> Node {
    let mut list: Node = None;
    for num in vec.into_iter().rev() {
        let mut temp = ListNode::new(num);
        temp.next = list;
        list = Some(Box::new(temp));
    }
    list
}

fn to_vec(list: &Node) -> Vec<i32> {
    let mut vec = Vec::new();
    if let Some(node) = list {
        vec.push(node.val);
        vec.append(&mut to_vec(&node.next));
    }
    vec
}

// given a list that contains at least two nodes, find the midpoint and split the list, then
// return the heads of both sublists
// unsafe fn split_list(head: &mut Node) -> (&mut Node, &mut Node) {
//     let mut fast: *mut Node = &mut head.as_mut().unwrap().next;
//     if fast.as_mut().unwrap().as_mut().unwrap().next.is_none() {
//         head.as_mut().unwrap().next = None;
//         return (head, fast.as_mut().unwrap());
//     }
//     let mut slow: *mut Node = fast.as_mut().unwrap();
//     loop {
//         let first = &mut fast.as_mut().unwrap().as_mut().unwrap().next;
//         if first.is_none() { break; }
//         fast = first;
//         let second = &mut fast.as_mut().unwrap().as_mut().unwrap().next;
//         if second.is_none() { break; }
//         slow = &mut slow.as_mut().unwrap().as_mut().unwrap().next;
//         fast = second;
//     }
//     fast = &mut slow.as_mut().unwrap().as_mut().unwrap().next;
//     slow.as_mut().unwrap().as_mut().unwrap().next = None;
//     (head, fast.as_mut().unwrap())
// }

unsafe fn split_list(head: &mut Node) -> (&mut Node, Node) {
    let mut fast: *const Node = &head.as_ref().unwrap().next;
    if fast.as_ref().unwrap().as_ref().unwrap().next.is_none() {
        let split = head.as_mut().unwrap().next.take();
        return (head, split);
    }
    fast = &fast.as_ref().unwrap().as_ref().unwrap().next;
    let mut slow: *mut Node = &mut head.as_mut().unwrap().next;
    loop {
        if fast.as_ref().unwrap().as_ref().unwrap().next.is_none() { break; }
        fast = &fast.as_ref().unwrap().as_ref().unwrap().next;
        if fast.as_ref().unwrap().as_ref().unwrap().next.is_none() { break; }
        fast = &fast.as_ref().unwrap().as_ref().unwrap().next;
        slow = &mut slow.as_mut().unwrap().as_mut().unwrap().next;
    }
    let split = slow.as_mut().unwrap().as_mut().unwrap().next.take();
    (head, split)
}

// given two sorted lists containing at least one node, merge them such that a is the new head
fn merge_lists(mut head: &mut Node, split: &mut Node) {
    // swap the contents of split with nodes in head of greater value, then iterate through head
    // to find another node of greater value. When head is exhausted, swap the contents of split
    // for the tail end of the head sublist
    while split.is_some() {
        if head.is_none() || head.as_ref().unwrap().val > split.as_ref().unwrap().val {
            let mut temp = head.take();
            temp = if let Some(head_contents) = temp { 
                split.replace(head_contents)
            } else { split.take() };
            head.replace(temp.unwrap());
        } else {
            let split_val = split.as_ref().unwrap().val;
            while head.is_some() && head.as_ref().unwrap().val <= split_val {
                head = &mut head.as_mut().unwrap().next;
            }
        }
    }
}

// given a list, sort its contents such that 
fn merge_sort(head: &mut Node) {
    if head.is_none() || head.as_ref().unwrap().next.is_none() { return; }
    let (head, mut split) = unsafe {
        split_list(head)
    };
    merge_sort(head);
    merge_sort(&mut split);
    merge_lists(head, &mut split);
}

pub fn sort_list(mut head: Node) -> Node {
    merge_sort(&mut head);
    head
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort_list() {
        let test_tuples = vec![
            (vec![1], vec![1]),
            (vec![2, 1], vec![1, 2]),
            (vec![5, 4, 3, 2, 1, 0], vec![0, 1, 2, 3, 4, 5]),
        ];
        for (init, expected) in test_tuples {
            assert_eq!(to_vec(&sort_list(from_vec(init))), expected);
        }
    }

    #[test]
    fn test_merge_lists() {
        let test_tuples = vec![
            (vec![1], vec![1], vec![1, 1]),
            (vec![2, 4], vec![1, 3], vec![1, 2, 3, 4]),
            (vec![1, 4], vec![2, 3], vec![1, 2, 3, 4]),
            (vec![2, 3], vec![1, 4], vec![1, 2, 3, 4]),
        ];
        for (a, b, expected) in test_tuples {
            let mut list_a = from_vec(a);
            let mut list_b = from_vec(b);
            merge_lists(&mut list_a, &mut list_b);
            assert_eq!(to_vec(&list_a), expected);
        }
    }

    #[test]
    fn test_split_first_half() {
        let test_tuples = vec![
            (vec![1, 2], vec![1]),
            (vec![1, 2, 3, 4], vec![1, 2]),
        ];
        for (list, head) in test_tuples {
            let mut list = from_vec(list);
            let (a, _) = unsafe { split_list(&mut list) };
            assert_eq!(to_vec(a), head);
        }
    }

    #[test]
    fn test_split_second_half() {
        let test_tuples = vec![
            (vec![1, 2], vec![2]),
            (vec![1, 2, 3, 4], vec![3, 4]),
        ];
        for (list, split) in test_tuples {
            let mut list = from_vec(list);
            let (_, b) = unsafe { split_list(&mut list) };
            assert_eq!(to_vec(&b), split);
        }
    }

    #[test]
    fn test_split_list() {
        let test_tuples = vec![
            (vec![1, 2], vec![1], vec![2]),
            (vec![1, 2, 3, 4], vec![1, 2], vec![3, 4]),
        ];
        for (list, head, split) in test_tuples {
            let mut list = from_vec(list);
            let (a, b) = unsafe { split_list(&mut list) };
            assert_eq!(to_vec(a), head);
            assert_eq!(to_vec(&b), split);
        }
    }
}
