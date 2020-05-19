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

const treeC = new BinaryTree(1);
treeC.left = new BinaryTree(2);

const treeD = new BinaryTree(3);
treeD.left = new BinaryTree(2);
treeD.right = new BinaryTree(4);
treeD.left.left = new BinaryTree(1);
treeD.right.right = new BinaryTree(5);

const treeE = new BinaryTree(50);
treeE.left = new BinaryTree(30);
treeE.left.left = new BinaryTree(20);
treeE.left.right = new BinaryTree(60);
treeE.right = new BinaryTree(80);
treeE.right.left = new BinaryTree(70);
treeE.right.right = new BinaryTree(90);

const treeF = new BinaryTree(1);
treeF.left = new BinaryTree(2);
treeF.left.left = new BinaryTree(4);
treeF.left.left.left = new BinaryTree(8);
treeF.left.right = new BinaryTree(5);
treeF.right = new BinaryTree(3);
treeF.right.left = new BinaryTree(6);
treeF.right.left.left = new BinaryTree(9);
treeF.right.left.right = new BinaryTree(10);
treeF.right.left.left.left = new BinaryTree(12);
treeF.right.right = new BinaryTree(7);
treeF.right.right.left = new BinaryTree(11);

const testTrees = [treeA, treeB, treeC, treeD, treeE, treeF];

describe('print', () => {
  const expected = [
    [1, 2, 3, null, null, 4, null],
    [1, 2, 3, null, null, 4, null, null, null, null, null, null, 5, null, null],
    [1, 2, null],
    [3, 2, 4, 1, null, null, 5],
    [50, 30, 80, 20, 60, 70, 90],
    [1, 2, 3, 4, 5, 6, 7, 8, null, null, null, 9, 10, 11, null,
      null, null, null, null, null, null, null, null, 12, null, null, null, null, null, null, null],
  ];
  test('it should return an array of node values', () => {
    testTrees.forEach((tree, index) => {
      expect(tree.print()).toEqual(expected[index]);
    });
  });
});

describe('height', () => {
  const expected = [2, 3, 1, 2, 2, 4];
  test('it should return the height of the tree', () => {
    testTrees.forEach((tree, index) => {
      expect(tree.height()).toBe(expected[index]);
    });
  });
});

describe('recursiveHeight', () => {
  const expected = [2, 3, 1, 2, 2, 4];
  test('it should return the height of the tree', () => {
    testTrees.forEach((tree, index) => {
      expect(tree.recursiveHeight()).toBe(expected[index]);
    });
  });
});

describe('isBalanced', () => {
  const expected = [true, false, true, true, true, true];
  test('it should return whether or not the tree is balanced', () => {
    testTrees.forEach((tree, index) => {
      expect(tree.isBalanced()).toBe(expected[index]);
    });
  });
});

describe('isSuperBalanced', () => {
  const expected = [true, false, true, true, true, false];
  test('it should return whether the tree is superbalanced', () => {
    testTrees.forEach((tree, index) => {
      expect(BinaryTree.isSuperBalanced(tree)).toBe(expected[index]);
    });
  });
});

describe('isValidBinarySearchTree', () => {
  const expected = [false, false, false, true, false, false];
  test('it should return whether the tree is a valid binary search tree', () => {
    testTrees.forEach((tree, index) => {
      expect(BinaryTree.isValidBinarySearchTree(tree)).toBe(expected[index]);
    });
  });
});
