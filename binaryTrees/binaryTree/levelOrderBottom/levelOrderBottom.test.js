const { levelOrderBottom } = require('./levelOrderBottom');
const { BinaryTree } = require('../binaryTree.js');

describe('levelOrderBottom', () => {
  test('it should return an empty array if provided a null root', () => {
    expect(levelOrderBottom(null)).toEqual([]);
  });

  test('it should return the values in bottom up level order', () => {
    const treeA = new BinaryTree(1);
    treeA.left = new BinaryTree(2);
    treeA.right = new BinaryTree(3);

    const treeB = new BinaryTree(1);
    treeB.right = new BinaryTree(3);
    treeB.right.right = new BinaryTree(7);
    expect(levelOrderBottom(treeA)).toEqual([[2, 3], [1]]);
    expect(levelOrderBottom(treeB)).toEqual([[7], [3], [1]]);
  });
});
