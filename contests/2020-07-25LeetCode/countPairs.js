/*
Given the root of a binary tree and an integer distance. A pair of two different leaf nodes of a
binary tree is said to be good if the length of the shortest path between them is less than or
equal to distance.

Return the number of good leaf node pairs in the tree.
*/
class TreeNode {
  constructor(val = 0, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}


const countPairs = (root, distance) => {
  let count = 0;
  const recursiveHelper = (node) => {
    const output = [...Array(distance)].map(() => 0);
    if (!node) return output;
    const { left, right } = node;
    if (!left && !right) {
      // one node at distance 1
      output[0] = 1;
      return output;
    }
    const leftLeaves = recursiveHelper(left);
    const rightLeaves = recursiveHelper(right);
    for (let i = 0; i < distance - 1; i += 1) {
      count += leftLeaves[i] * rightLeaves[distance - i - 1];
    }
    for (let i = 1; i < distance; i += 1) {
      output[i] = leftLeaves[i - 1] + rightLeaves[i - 1];
    }
    return output;
  };
  recursiveHelper(root);
  return count;
};

const testTree = new TreeNode(1, new TreeNode(2, new TreeNode(4), new TreeNode(5)),
  new TreeNode(6), new TreeNode(7));
const testDist = 3;
const output = countPairs(testTree, testDist);
