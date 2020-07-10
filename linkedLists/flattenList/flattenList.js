/*
You are given a doubly linked list which in addition to the next and previous pointers, it could
have a child pointer, which may or may not point to a separate doubly linked list. These child
lists may have one or more children of their own, and so on, to produce a multilevel data
structure, as shown in the example below.

Flatten the list so that all the nodes appear in a single-level, doubly linked list. You are
given the head of the first level of the list.
*/

/*
Multilevel Doubly Linked List
class Node {
  constructor(val=null, prev=null, next=null, child=null) {
    this.val = val;
    this.prev = prev;
    this.next = next;
    this.child = child;
  }
}
*/

const flatten = (head) => {
  if (!head) return head;
  const parentStack = [[head, null, null]];
  let p1 = head;
  let p2 = head;
  while (parentStack.length) {
    // advance parent pointer through list until a node with a child list is found
    while (p2.next && !p2.child) {
      p2 = p2.next;
    }
    if (p2.child) {
      // push parent pointer and parent.next to the stack
      parentStack.push([p1, p2, p2.next]);
      // move parent pointer to child and repeat the last two steps if a child is found
      p1 = p2.child;
      [p2.child, p2] = [null, p2.child];
      // if no child is found and there is no next node,
    } else {
      // pop the stack and splice in the head and tail to the popped nodes
      const [lastHead, lastParent, lastParentNext] = parentStack.pop();
      p1.prev = lastParent;
      p2.next = lastParentNext;
      if (lastParentNext) lastParentNext.prev = p2;
      if (lastParent) {
        lastParent.next = p1;
        p1 = lastHead;
      }
    }
  }
  return head;
};

module.exports = { flatten };
