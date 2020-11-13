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

const connectAcross = (left, right) => {
  if (left === null || right === null) return;
  left.next = right;
  connectAcross(left.right, right.left);
};

const connect = (root) => {
  if (root === null) return root;
  connectAcross(root.left, root.right);
  connect(root.left);
  connect(root.right);
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

module.exports = { connect };
