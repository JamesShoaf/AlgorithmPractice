/* eslint-disable no-param-reassign */
/*
You are given a perfect binary tree where all leaves are on the same level, and every parent has
two children. The binary tree has the following definition:

class Node {
  constructor(val, left, right, next) {
    this.val = val === undefined ? null : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
    this.next = next === undefined ? null : next;
  }
}

Populate each next pointer to point to its next right node. If there is no next right node, the
next pointer should be set to NULL.

Initially, all next pointers are set to NULL.
*/

const perfectConnectAcross = (left, right) => {
  if (left === null || right === null) return;
  left.next = right;
  perfectConnectAcross(left.right, right.left);
};

const perfectConnect = (root) => {
  if (root === null) return root;
  perfectConnectAcross(root.left, root.right);
  perfectConnect(root.left);
  perfectConnect(root.right);
  return root;
};

/*
Other solutions for comparison
#1 solution
var connect = function(root) {
   if(!root) return null;
    const queue = [root];
    while(queue.length){
        const size = queue.length;
        const level = queue.slice();

        for(let i=0; i< size; i++){
            const current = queue.shift();
            current.next = level[i + 1];
            if(current.left) queue.push(current.left);
            if(current.right) queue.push(current.right);
        }
    }
   return root;
};

single function recursive implementation
var connect = function(root) {
    if(!root || !root.left || !root.right) {
        return root;
    }
    root.left.next = root.right;
    if(root.next) {
        root.right.next = root.next.left;
    }
    connect(root.left);
    connect(root.right);
    return root;
};
*/

/*
If the tree is imperfect, the algorithm needs to incorporate a few additional checks
*/
// initial Level-order traversal (O(n) time, O(n) space)
// const connect = (root) => {
//   if (!root) return root;
//   let level = [root];
//   while (level.length > 0) {
//     const nextLevel = [];
//     for (let i = 0; i < level.length; i += 1) {
//       const node = level[i];
//       if (level[i + 1]) node.next = level[i] + 1;
//       if (node.left) nextLevel.push(node.left);
//       if (node.right) nextLevel.push(node.right);
//     }
//     level = nextLevel;
//   }
//   return root;
// };

// Pointer based refactor of Level-order traversal (O(n) time, O(1) space)

// travels right until it finds a node with a child, then returns a reference to that node's
// leftmost child
const leftmostNext = (node) => {
  let next = node;
  while (next) {
    if (next.left) return next.left;
    if (next.right) return next.right;
    next = next.next;
  }
  return next;
};

const connect = (root) => {
  // leftmost node of current level
  let leftmost = root;
  while (leftmost) {
    let curr = leftmost;
    while (curr) {
      // connect current node's children to next node to their right
      if (curr.left) curr.left.next = curr.right || leftmostNext(curr.next);
      if (curr.right) curr.right.next = leftmostNext(curr.next);
      // move current pointer right
      curr = curr.next;
    }
    // move leftmost pointer to leftmost of next level
    leftmost = leftmostNext(leftmost);
  }
  return root;
};

module.exports = { perfectConnect, connect };
