const { BinaryTree } = require('./binaryTree');

class BalancedBinarySearchTree extends BinaryTree {
  constructor(val) {
    super(val);
    this.parent = null;
  }

  root() {
    let root = this;
    while (root.parent) root = root.parent;
    return root;
  }

  rotateRight() {
    const { left, right, parent } = this;
    if (!parent) return left.rotateRight();
    const grandparent = parent.parent;
    parent.parent = this;
    this.parent = grandparent;
    if (grandparent) {
      if (grandparent.left === parent) grandparent.left = this;
      else grandparent.right = this;
    }
    if (parent.left === this) {
      if (right) right.parent = parent;
      parent.left = right;
      this.right = parent;
      return this;
    }
    if (left) left.parent = parent;
    parent.right = left;
    this.left = parent;
    return this;
  }
}

module.exports = { BalancedBinarySearchTree };
