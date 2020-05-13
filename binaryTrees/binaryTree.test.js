const { BinaryTree } = require('./binaryTree');

const treeA = new BinaryTree(1);
treeA.left = new BinaryTree(2);
treeA.right = new BinaryTree(3);
treeA.right.left = new BinaryTree(4);

const treeB = new BinaryTree(1);
treeB.left = new BinaryTree(2);
treeB.right = new BinaryTree(3);
treeB.right.left = new BinaryTree(4);
treeB.right.left.right = new BinaryTree(5);

const testTrees = [treeA, treeB];

describe('print', () => {
  const expected = [
    [1, 2, 3, null, null, 4, null],
    [1, 2, 3, null, null, 4, null, null, null, null, null, null, 5, null, null],
  ];
  test('it should return an array of node values', () => {
    testTrees.forEach((tree, index) => {
      expect(tree.print()).toEqual(expected[index]);
    });
  });
});

describe('isSuperBalanced', () => {
  const expected = [true, false];
  test('it should return whether the tree is superbalanced', () => {
    testTrees.forEach((tree, index) => {
      expect(BinaryTree.isSuperBalanced(tree)).toBe(expected[index]);
    });
  });
});
