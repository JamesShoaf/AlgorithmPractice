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

struct Solution {}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut filtered_values: Vec<i32> = Vec::new();
        let mut current = head;
        loop {
            let current_clone = current.clone();
            match current_clone {
                None => break,
                Some(boxed_node) => {
                    let node = *boxed_node;
                    if node.val != val { filtered_values.push(node.val); }
                    current = node.next;
                }
            }
        }
        let mut output: Option<Box<ListNode>> = None;
        while filtered_values.len() > 0 {
            if let Some(value) = filtered_values.pop() {
                let mut node = ListNode::new(value);
                node.next = output.clone();
                output = Some(Box::new(node));
            }
        }
        output
    }
    // pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    //     // set a new pointer to the head
    //     let mut start: Option<Box<ListNode>> = head.clone();
    //     // while the value of the pointer's node is the value to be removed, move the pointer ahead
    //     loop {
    //         let start_clone = start.clone();
    //         match start_clone {
    //             None => break,
    //             Some(boxed_node) => {
    //                 let node: ListNode = *boxed_node;
    //                 if node.val == val { start = node.next; } else { break; }
    //             }
    //         }
    //     }
    //     // if no valid starting pointer is found, return None
    //     if start.is_none() { return None; }
    //     // otherwise, hold onto the starting pointer to return it in the last step
    //     // set a mutable reference, prev, to the starting pointer
    //     let mut prev: Option<Box<ListNode>> = start.clone();
    //     // while the prev pointer points to a node
    //     loop {
    //         match prev {
    //             None => break,
    //             Some(boxed_prev) => {
    //                 // set a reference, next, to the starting pointer's next node
    //                 let mut prev_node: ListNode = *boxed_prev;
    //                 let mut next: Option<Box<ListNode>> = prev_node.next;
    //                 prev_node.next = loop {
    //                     // advance the pointer until a value other than the one to be removed is found
    //                     let next_clone = next.clone()
    //                     match next_clone {
    //                         None => break None,
    //                         Some(boxed_next) => {
    //                             let next_node: ListNode = *boxed_next;
    //                             if next_node.val == val { next = next_node.next; }
    //                             else { break next; }
    //                         }
    //                     }
    //                 };
    //                 // set the prev node's next value to next
    //                 prev_node.val += 1;
    //                 prev_node.next = next.clone();
    //                 // and advance the prev pointer
    //                 prev = next;
    //             }
    //         }
    //     }
    //     // return the starting pointer
    //     start
    // }
}

fn main() {
    let mut one = ListNode::new(1);
    let mut two = ListNode::new(2);
    let mut six_one = ListNode::new(6);
    let mut three = ListNode::new(3);
    let mut four = ListNode::new(4);
    let mut five = ListNode::new(5);
    let six_two = ListNode::new(6);
    five.next = Some(Box::new(six_two));
    four.next = Some(Box::new(five));
    three.next = Some(Box::new(four));
    six_one.next = Some(Box::new(three));
    two.next = Some(Box::new(six_one));
    one.next = Some(Box::new(two));
    let mut output = Solution::remove_elements(Some(Box::new(one)), 6);
    loop {
        let output_clone = output.clone();
        match output_clone {
            None => break,
            Some(node) => {
                println!("node value: {}", node.val);
                output = node.next;
            }
        }
    }
}
