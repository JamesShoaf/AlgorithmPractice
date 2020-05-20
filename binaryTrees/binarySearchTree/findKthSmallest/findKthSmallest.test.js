const { BinarySearchTree: BST } = require('../binarySearchTree');
const { findKthSmallest } = require('./findKthSmallest');

describe('findKthSmallest', () => {
  const bst = new BST(10);
  bst.addToTree([9, 5, 6, 8, 12, 11, 13, 11]);
  test('it should return the kth smallest element of the tree', () => {
    const expected = [null, 5, 6, 8, 9, 10, 11, 11, 12, 13, null];
    for (let i = 0; i < 10; i += 1) {
      expect(findKthSmallest(bst, i)).toBe(expected[i]);
    }
  });
});
