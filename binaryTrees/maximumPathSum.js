class TreeNode {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

const maximumPathSum = (root) => {
  let maximum = 0;
  const recursivePathSearch = (tree) => {
    const leftPath = (tree.left === null) ? 0 : recursivePathSearch(tree.left);
    const rightPath = (tree.right === null) ? 0 : recursivePathSearch(tree.right);
    const value = tree.val;
    if (leftPath < 0 && rightPath < 0) {
      maximum = Math.max(maximum, value);
      return value;
    }
    maximum = Math.max(maximum, leftPath + value + rightPath);
    return value + Math.max(leftPath, rightPath);
  };
  recursivePathSearch(root);
  return maximum;
};
