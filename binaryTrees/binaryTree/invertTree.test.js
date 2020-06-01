const { BinaryTree } = require('./binaryTree');
const { invertTree } = require('./invertTree');

const testTrees = [];
const expected = [];

const treeA = new BinaryTree(0);
treeA.left = new BinaryTree(1);
treeA.left.right = new BinaryTree(2);
testTrees.push(treeA);
expected.push([0, null, 1, null, null, 2, null]);

const treeB = new BinaryTree(4);
treeB.left = new BinaryTree(2);
treeB.left.left = new BinaryTree(1);
treeB.left.right = new BinaryTree(3);
treeB.right = new BinaryTree(7);
treeB.right.left = new BinaryTree(6);
treeB.right.right = new BinaryTree(9);
testTrees.push(treeB);
expected.push([4, 7, 2, 9, 6, 3, 1]);

describe('invertTree', () => {
  test('it should invert the binary tree', () => {
    testTrees.forEach((root, index) => {
      expect(invertTree(root).print()).toEqual(expected[index]);
    });
  });
  test('it should return null if passed null', () => {
    expect(invertTree(null)).toBe(null);
  });
});
