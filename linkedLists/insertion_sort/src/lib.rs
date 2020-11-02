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

use std::mem;

fn helper(mut unsort_head: Node, mut sort_head: Node) -> Node {
    // base case: no more nodes to sort
    if unsort_head.is_none() { return sort_head; }
    let next_unsort = unsort_head.as_mut().unwrap().next.take();
    let unsort_val = unsort_head.as_ref().unwrap().val;
    let mut current = &mut sort_head;
    while current.is_some() && current.as_ref().unwrap().val < unsort_val {
        current = &mut current.as_mut().unwrap().next;
    }
    mem::swap(current, &mut unsort_head);
    mem::swap(&mut current.as_mut().unwrap().next, &mut unsort_head);
    helper(next_unsort, sort_head)
}

pub fn insertion_sort_list(head: Node) -> Node {
    helper(head, None)
}

/* #1 (non-vec) solution for comparison
pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }

    let mut dummy = Box::new(ListNode::new(-1));
    let mut prev = &mut dummy;

    while let Some(mut node) = head {
        let temp = node.next.take();

        /* Before insert, the prev is at the last node of the sorted list.
        Only the last node's value is larger than the current inserting node
        should we move the temp back to the head*/
        if prev.val > node.val {
            prev = &mut dummy;
        }
        //find the right place to insert
        while prev.next.is_some() && prev.next.as_ref().unwrap().val < node.val {
            prev = prev.next.as_mut().unwrap();
        }
        //insert between pre and pre.next
        node.next = prev.next.take();
        prev.next = Some(node);
        // prev = dummy; // Don't set prev to the head of the list after insert
        head = temp;
    }

    dummy.next
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}