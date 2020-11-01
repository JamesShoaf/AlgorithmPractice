/* 
Given head which is a reference node to a singly-linked list. The value of each node in the linked
list is either 0 or 1. The linked list holds the binary representation of a number.

Return the decimal value of the number in the linked list.
*/

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

fn helper(node: &ListNode, res: &mut i32) {
    *res <<= 1;
    *res += node.val; // |= is equivalent in the case of binary values
    if let Some(next) = &node.next {
        helper(&next, res)
    }
}

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut res = 0;
    if let Some(node) = head {
        helper(&node, &mut res);
    }
    res
}

/* 
equivalent iterative implementation
pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
    let mut val = 0;
    
    let mut node = &head;
    
    while let Some(n) = node {
        val <<= 1;
        val += n.val;
        node = &n.next;
    }
    
    val
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
