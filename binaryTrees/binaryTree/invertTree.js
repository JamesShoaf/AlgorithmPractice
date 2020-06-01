const invertTree = (root) => {
  const queue = [];
  if (root) queue.push(root);
  while (queue.length) {
    const node = queue.pop();
    if (node.left) queue.push(node.left);
    if (node.right) queue.push(node.right);
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
