class BinaryTree {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }

  print() {
    const queue = [this];
    let currentGenValues = [];
    let output = [this.val];
    let generation = 1;
    for (let index = 0; ; index += 1) {
      const currentNode = queue[index];
      if (currentNode === null) {
        queue.push(null, null);
        currentGenValues.push(null, null);
      } else {
        queue.push(currentNode.left, currentNode.right);
        if (currentNode.left === null) {
          currentGenValues.push(null);
        } else {
          currentGenValues.push(currentNode.left.val);
        }
        if (currentNode.right === null) {
          currentGenValues.push(null);
        } else {
          currentGenValues.push(currentNode.right.val);
        }
      }
      if (index === 2 ** generation - 2) {
        if (currentGenValues.every((val) => val === null)) {
          break;
        }
        output = output.concat(currentGenValues);
        currentGenValues = [];
        generation += 1;
      }
    }
    return output;
  }

  height() {
    let height = 0;
    const stack = [[this, 0]];
    while (stack.length) {
      const [node, nodeHeight] = stack.pop();
      height = Math.max(height, nodeHeight);
      if (node.left) stack.push([node.left, nodeHeight + 1]);
      if (node.right) stack.push([node.right, nodeHeight + 1]);
    }
    return height;
  }

  isBalanced() {
    const left = (this.left === null) ? -1 : this.left.height();
    const right = (this.right === null) ? -1 : this.right.height();
    if (Math.abs(left - right) > 1) return false;
    if (this.left !== null && !this.left.isBalanced()) return false;
    if (this.right !== null && !this.right.isBalanced()) return false;
    return true;
  }

  static isSuperBalanced(tree) {
    let minLeafDepth;
    let maxLeafDepth;
    let currentGeneration = [tree];
    for (let depth = 0; currentGeneration.length; depth += 1) {
      const { length } = currentGeneration;
      const nextGeneration = [];
      for (let i = 0; i < length; i += 1) {
        const currentNode = currentGeneration[i];
        const children = [];
        if (currentNode.left) children.push(currentNode.left);
        if (currentNode.right) children.push(currentNode.right);
        if (children.length === 0) {
          minLeafDepth = Math.min(minLeafDepth, depth) || depth;
          maxLeafDepth = Math.max(maxLeafDepth, depth) || depth;
          if (maxLeafDepth - minLeafDepth > 1) return false;
        }
        nextGeneration.push(...children);
      }
      currentGeneration = nextGeneration;
    }
    return true;
  }

  static isValidBinarySearchTree(tree) {
    const stack = [[tree, Number.NEGATIVE_INFINITY, Number.POSITIVE_INFINITY]];
    while (stack.length) {
      const [node, lowerBound, upperBound] = stack.pop();
      const { val, left, right } = node;
      if (val < lowerBound) return false;
      if (val > upperBound) return false;
      if (left !== null) stack.push([left, lowerBound, val]);
      if (right !== null) stack.push([right, val, upperBound]);
    }
    return true;
  }
}

module.exports = {
  BinaryTree,
};
