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
}

module.exports = { BalancedBinarySearchTree };
