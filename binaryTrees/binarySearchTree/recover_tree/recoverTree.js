/*
You are given the root of a binary search tree (BST), where exactly two nodes of the tree were
swapped by mistake. Recover the tree without changing its structure.

Unfortunately, the borrow checker has bested you today. Please code this exercise in JavaScript
*/

const recoverTree = (root) => {
  let current = root;
  const stack = [];
  let first = null;
  let second = null;
  let firstPair = false;
  let third = null;
  let fourth = null;
  while (current !== null || stack.length > 0) {
    while (current) {
      stack.push(current);
      current = current.left;
    }
    current = stack.pop();
    // do something with current value
    if (!firstPair) {
      if (second === null) second = current;
      else {
        first = second;
        second = current;
        if (second.val < first.val) {
          firstPair = true;
          fourth = second;
        }
      }
    } else {
      third = fourth;
      fourth = current;
      if (fourth.val < third.val) {
        [first.val, fourth.val] = [fourth.val, first.val];
        return;
      }
    }
    current = current.right;
  }
  [first.val, second.val] = [second.val, first.val];
};

module.exports = { recoverTree };
