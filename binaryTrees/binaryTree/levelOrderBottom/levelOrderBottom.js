const levelOrderBottom = (root) => {
  if (!root) return [];
  const queueValStack = [];
  let queue = [root];
  while (queue.length) {
    const newQueue = [];
    const queueVals = [];
    for (let i = 0; i < queue.length; i += 1) {
      const currentNode = queue[i];
      const { val, left, right } = currentNode;
      queueVals.push(val);
      if (left) newQueue.push(left);
      if (right) newQueue.push(right);
    }
    queueValStack.push(queueVals);
    queue = newQueue;
  }
  const output = [];
  while (queueValStack.length) output.push(queueValStack.pop());
  return output;
};

module.exports = { levelOrderBottom };
