const findKthSmallest = (root, k) => {
  if (k < 1) return null;

  let valuesCounted = 0;
  let currentNode;
  let nextNode = root;
  const nodeStack = [];

  while (nextNode !== currentNode && valuesCounted < k) {
    // move to next node
    currentNode = nextNode;
    const { left, right, val } = currentNode;
    // if there is a node further left
    if (left && currentNode !== nodeStack[nodeStack.length - 1]) {
      // add the current node to a stack to return to later
      nodeStack.push(currentNode);
      // and go left
      nextNode = left;
    } else {
      // if returning to the node, skip going left and pop the node off the stack
      if (currentNode === nodeStack[nodeStack.length - 1]) {
        nodeStack.pop();
      }
      // increment the counter of traversed nodes
      valuesCounted += 1;
      // and return the current value if applicable
      if (valuesCounted === k) return val;
      // go right if possible
      if (right) {
        nextNode = right;
        // otherwise, start backtracking
      } else if (nodeStack.length) {
        nextNode = nodeStack[nodeStack.length - 1];
      }
    }
  }
  // if k is greater than the number of nodes
  return currentNode.val;
};

// implementation using Morris traversal
// const findKthSmallest = (root, k) => {
//   if (!Number.isInteger(k) || k < 1 || root === null) return null;
//   let valuesCounted = 0;
//   let current = root;
//   while (current !== null) {
//     if (current.left === null) {
//       valuesCounted += 1;
//       if (valuesCounted === k) return current.val;
//       current = current.right;
//     } else {
//       let pre = current.left;
//       while (pre.right !== null && pre.right !== current) pre = pre.right;
//       if (pre.right === null) {
//         pre.right = current;
//         current = current.left;
//       } else {
//         pre.right = null;
//         valuesCounted += 1;
//         if (valuesCounted === k) return current.val;
//         current = current.right;
//       }
//     }
//   }
//   return null;
// };

module.exports = { findKthSmallest };
