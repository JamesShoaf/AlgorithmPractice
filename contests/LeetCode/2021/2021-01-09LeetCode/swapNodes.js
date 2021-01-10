/* You are given the head of a linked list, and an integer k.

Return the head of the linked list after swapping the values of the kth node from the beginning and the kth node from the end (the list is 1-indexed). */

const getLength = (head) => {
  let len = 0;
  let node = head;
  while (node) {
    node = node.next;
    len += 1;
  }
  return len;
};

const swapNodes = (head, k) => {
  const length = getLength(head);
  let node1 = head;
  for (let i = 1; i < k; i += 1) {
    node1 = node1.next;
  }
  let node2 = head;
  for (let i = 0; i < length - k; i += 1) {
    node2 = node2.next;
  }
  [node1.val, node2.val] = [node2.val, node1.val];
  return head;
};
