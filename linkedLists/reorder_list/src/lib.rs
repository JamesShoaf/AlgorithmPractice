// Definition for singly-linked list.
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

pub fn count_nodes(mut head: &mut Option<Box<ListNode>>) -> usize {
    let mut count = 0;
    while let Some(node) = head {
        count += 1;
        head = &mut node.next;
    } 
    count
}

pub fn split_at_midpoint(mut head: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
    let node_count = count_nodes(&mut head);
    for _ in 0..((node_count + 1)/ 2) {
        if let Some(node) = head {
            head = &mut node.next;
        }
    }
    if let Some(mut inner) = head.take() {
        let temp = ListNode::new(0);
        let mut temp = Some(Box::new(temp));
        head = &mut temp;
        head = &mut None;
        head.replace(inner);
        // let next = &mut inner.next;
        // inner.next = None;
        // head.replace(inner);
        // head = next;
    }
    head
}

pub fn reverse_list(head: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    let mut current_node: Option<Box<ListNode>> = head.take();
    while let Some(mut current_node_inner) = current_node.take() {
        let next = current_node_inner.next.take();
        current_node_inner.next = prev.take();
        prev = Some(current_node_inner);
        current_node = next;
    }
    if let Some(prev_inner) = prev.take() {
        head.replace(prev_inner);
    }
    head
}

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    // find middle of list
    let mut midpoint = split_at_midpoint(head);
    // reverse second half of list
    midpoint = reverse_list(midpoint);
    // zip first half with reversed second half
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
