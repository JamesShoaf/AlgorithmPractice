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
}

module.exports = {
  BinaryTree,
};
