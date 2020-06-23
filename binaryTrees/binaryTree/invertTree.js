const invertTree = (root) => {
  const stack = [];
  if (root) stack.push(root);
  while (stack.length) {
    const node = stack.pop();
    if (node.left) stack.push(node.left);
    if (node.right) stack.push(node.right);
    [node.left, node.right] = [node.right, node.left];
  }
  return root;
};


/*
recursive implementation
const invertTree = (root) => {
  if (root) [root.left, root.right] = [invertTree(root.right), invertTree(root.left)];
  return root;
};
*/


module.exports = { invertTree };
