#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/*
You are given two non-empty linked lists representing two non-negative integers. The most
significant digit comes first and each of their nodes contain a single digit. Add the two numbers
and return it as a linked list.

You cannot modify the input lists

You may assume the two numbers do not contain any leading zero, except the number 0 itself.
*/

type List = Option<Box<ListNode>>;

fn find_length(mut list: &List) -> usize {
    let mut len = 0;
    while let Some(node) = list {
        len += 1;
        list = &node.next;
    }
    len
}

// reduce the remaining node count, increment the sum by the current node's value, then advance the pointer
fn get_next_node<'a>(list: &'a List, len: &mut usize, sum: &mut i32) -> &'a List {
    if let Some(node) = list {
        *sum += node.val;
        *len -= 1;
        return &node.next;
    }
    list
}

fn recursive_helper(
    mut long: &List,
    mut l_len: usize,
    mut short: &List,
    mut s_len: usize,
    carry: &mut i32,
) -> List {
    // base case: terminate recursion when long is empty
    Some(()).filter(|_| l_len == 0)?;
    let mut sum = 0;
    if l_len == s_len {
        short = get_next_node(short, &mut s_len, &mut sum);
    }
    long = get_next_node(long, &mut l_len, &mut sum);
    let next = recursive_helper(long, l_len, short, s_len, &mut sum);
    // 1s place of sum becomes node value
    let mut res = ListNode::new(sum % 10);
    res.next = next;
    // and 10s place of sum increments carry
    *carry += sum / 10;
    Some(Box::new(res))
}

pub fn add_two_numbers(l1: List, l2: List) -> List {
    let (len1, len2) = (find_length(&l1), find_length(&l2));
    let mut carry = 0;
    let list = if len1 >= len2 {
        recursive_helper(&l1, len1, &l2, len2, &mut carry)
    } else {
        recursive_helper(&l2, len2, &l1, len1, &mut carry)
    };
    // add extra node if carry bit
    if carry == 1 {
        let mut res = ListNode::new(carry);
        res.next = list;
        return Some(Box::new(res));
    }
    list
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
