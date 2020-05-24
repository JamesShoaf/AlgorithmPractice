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

// const bstFromPreorder = (preOrder) => {
//   const root = new TreeNode(preOrder[0]);
//   const { length } = preOrder;
//   for (let i = 1; i < length; i += 1) {
//     const val = preOrder[i];
//     root.addToTree(val);
//   }
//   return root;
// };

const bstFromPreorder = (preorder) => {
  // the first node of a pre-order is the root
  const root = new TreeNode(preorder[0]);
  // add the root as the first node to check pre-order values against
  const stack = [root];
  // iterate through values in the preorder
  const { length } = preorder;
  for (let i = 1; i < length; i += 1) {
    // create new node
    const nextNode = new TreeNode(preorder[i]);
    // reset the reference to the current sub-tree
    let subRoot = null;
    // remove small nodes from the stack to find the greatest node less than the current value
    while (stack.length && preorder[i] > stack[stack.length - 1].val) subRoot = stack.pop();
    // if there is such a node, add the new node to its right
    stack.push(subRoot
      ? subRoot.right = nextNode
      : stack[stack.length - 1].left = nextNode);
    // if the value is less than all nodes in the stack, set the node as the left child of the least
    // in either case, push the node to the stack
  }
  return root;
};

module.exports = {
  TreeNode,
  bstFromPreorder,
};
