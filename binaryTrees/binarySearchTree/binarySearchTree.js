const {
  BinaryTree,
} = require('../binaryTree/binaryTree');

class BinarySearchTree extends BinaryTree {
  addToTree(val) {
    if (Array.isArray(val)) {
      const [value, ...rest] = val;
      const valNode = this.addToTree(value);
      return (rest.length) ? this.addToTree(rest) : valNode;
    }
    const node = (val instanceof BinarySearchTree)
      ? val
      : new BinarySearchTree(val);
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
