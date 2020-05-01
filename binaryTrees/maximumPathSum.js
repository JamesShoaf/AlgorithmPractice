// class BinaryTree {
//   constructor(val) {
//     this.val = val;
//     this.left = null;
//     this.right = null;
//   }
// }

const maximumPathSum = (root) => {
  let maximum = root.val;
  const recursivePathSearch = (tree) => {
    if (tree === null) return 0;
    const leftPath = recursivePathSearch(tree.left);
    const rightPath = recursivePathSearch(tree.right);
    const value = tree.val;
    maximum = Math.max(
      maximum,
      value,
      value + leftPath,
      value + rightPath,
      value + leftPath + rightPath,
    );
    return (leftPath < 0 && rightPath < 0)
      ? value
      : value + Math.max(leftPath, rightPath);
  };
  recursivePathSearch(root);
  return maximum;
};

module.exports = {
  maximumPathSum,
};
