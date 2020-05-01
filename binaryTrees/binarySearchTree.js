const {
  BinaryTree,
} = require('./binaryTree');

class BinarySearchTree extends BinaryTree {
  addToTree(val) {
    const node = (val instanceof BinaryTree)
      ? val
      : new BinaryTree(val);
    if (node.val < this.val) {
      if (this.left === null) {
        this.left = node;
        return node;
      }
      return this.left.addToTree(node);
    }
    if (this.right === null) {
      this.right = node;
      return this.right;
    }
    return this.right.addToTree(node);
  }
}

module.exports = {
  BinarySearchTree,
};
