const { BalancedBinarySearchTree: BBST } = require('./balancedBinarySearchTree');

describe('BBST', () => {
  let bbst;

  beforeEach(() => {
    bbst = new BBST(5);
  });

  describe('rotateLeft', () => {
    test('it should rotate nodes left', () => {
      const seven = new BBST(7);
      bbst.right = seven;
      seven.parent = bbst;
      const nine = new BBST(9);
      seven.right = nine;
      nine.parent = seven;
      const eight = new BBST(8);
      nine.left = eight;
      eight.parent = nine;
      const eleven = new BBST(11);
      nine.right = eleven;
      eleven.parent = nine;

      expect(nine.rotateLeft()).toBe(nine);
      expect(nine.parent).toBe(bbst);
      expect(bbst.right).toBe(nine);
      expect(nine.left).toBe(seven);
      expect(seven.parent).toBe(nine);
      expect(seven.right).toBe(eight);
      expect(eight.parent).toBe(seven);
    });
  });

  describe('rotateRight', () => {
    test('it should rotate nodes right', () => {
      const four = new BBST(4);
      bbst.left = four;
      four.parent = bbst;
      const two = new BBST(2);
      four.left = two;
      two.parent = four;
      const three = new BBST(3);
      two.right = three;
      three.parent = two;
      const one = new BBST(1);
      two.left = one;
      one.parent = two;

      expect(two.rotateRight()).toBe(two);
      expect(two.parent).toBe(bbst);
      expect(bbst.left).toBe(two);
      expect(two.right).toBe(four);
      expect(four.parent).toBe(two);
      expect(four.left).toBe(three);
      expect(three.parent).toBe(four);
    });
  });

  describe('root', () => {
    test('it should return the root node', () => {
      expect(bbst.root()).toBe(bbst);
      const four = new BBST(4);
      bbst.parent = four;
      expect(bbst.root()).toBe(four);
      const three = new BBST(3);
      four.parent = three;
      expect(bbst.root()).toBe(three);
    });
  });

  describe('addToTree', () => {
    test('it should add elements to the tree', () => {
      expect(bbst.addToTree(8)).toBe(bbst.right);
      expect(bbst.right.val).toBe(8);
      expect(bbst.right.parent).toBe(bbst);
    });

    test('it should balance the tree with a single right rotation', () => {
      bbst.addToTree(8);
      expect(bbst.root().print()).toEqual([5, null, 8]);
      bbst.addToTree(11);
      expect(bbst.root().print()).toEqual([8, 5, 11]);
    });

    test('it should balance the tree with a single left rotation', () => {
      bbst.addToTree(3);
      expect(bbst.root().print()).toEqual([5, 3, null]);
      bbst.addToTree(1);
      expect(bbst.root().print()).toEqual([3, 1, 5]);
    });

    test('it should balance the tree with a right left double rotation', () => {
      bbst.addToTree(8);
      expect(bbst.root().print()).toEqual([5, null, 8]);
      bbst.addToTree(6);
      expect(bbst.root().print()).toEqual([6, 5, 8]);
    });

    test('it should balance the tree with a left right double rotation', () => {
      bbst.addToTree(1);
      expect(bbst.root().print()).toEqual([5, 1, null]);
      bbst.addToTree(3);
      expect(bbst.root().print()).toEqual(3, 1, 5);
    });
  });
});
