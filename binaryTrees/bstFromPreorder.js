class TreeNode {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }

  addToTree(val) {
    const node = (val instanceof TreeNode)
      ? val
      : new TreeNode(val);
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

const bstFromPreorder = (preOrder) => {
  const root = new TreeNode(preOrder[0]);
  const { length } = preOrder;
  for (let i = 1; i < length; i += 1) {
    const val = preOrder[i];
    root.addToTree(val);
  }
  return root;
};

module.exports = {
  TreeNode,
  bstFromPreorder,
};
