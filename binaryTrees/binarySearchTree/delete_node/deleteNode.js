/* eslint-disable no-param-reassign */
// class BSTNode {
//   constructor(val = 0, left = null, right = null) {
//     this.val = val;
//     this.left = left;
//     this.right = right;
//   }
// }

const deleteNode = (root, key) => {
  if (!root) return root;
  if (root.val > key) root.left = deleteNode(root.left, key);
  if (root.val < key) root.right = deleteNode(root.right, key);
  if (root.val === key) {
    const { left } = root;
    const { right } = root;
    if (left) {
      let temp = left;
      while (temp.right) temp = temp.right;
      temp.right = right;
      return left;
    }
    return right;
  }
  return root;
};
