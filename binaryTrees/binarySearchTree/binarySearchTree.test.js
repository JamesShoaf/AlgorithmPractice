const { BinarySearchTree: BST } = require('./binarySearchTree');

describe('BST', () => {
  let bst;

  beforeEach(() => {
    bst = new BST(5);
  });

  describe('addToTree', () => {
    test('it should add lesser nodes to the left branch', () => {
      bst.addToTree(3);
      expect(bst.print()).toEqual([5, 3, null]);
    });

    test('it should add higher nodes to the right branch', () => {
      bst.addToTree(7);
      expect(bst.print()).toEqual([5, null, 7]);
    });

    test('it should add multiple nodes when given an array of values', () => {
      bst.addToTree([9, 12, 11, 3, 4]);
      expect(bst.print()).toEqual([5, 3, 9, null, 4, null, 12,
        null, null, null, null, null, null, 11, null]);
    });
  });
});
