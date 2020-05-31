const { maxProduct } = require('./maxProduct');
const { maxArea } = require('./maxArea');
const { minReorder } = require('./minReorder');
// const { solution4 } = require('./solution4');


describe('maxProduct', () => {
  const testTuples = [
    [
      [3,4,5,2],
      12,
    ],
    [
      [1,5,4,5],
      16,
    ],
    [
      [3,7],
      12,
    ],
  ];

  test('it should return the correct product', () => {
    testTuples.forEach(([nums, expected]) => {
      expect(maxProduct(nums)).toBe(expected);
    });
  });
});


describe('maxArea', () => {
  const testTuples = [
    [
      5,
      4,
      [1, 2, 4],
      [1, 3],
      4,
    ],
    [
      5,
      4,
      [3, 1],
      [1],
      6,
    ],
    [
      5,
      4,
      [3],
      [3],
      9,
    ],
    [
      6,
      5,
      [5],
      [2,1,4],
      10,
    ],
    [
      12,
      44,
      [2,1,9,6,7],
      [35,5,21,12,4,7,18,32,36,40,10,30,24,17,8,20,9,3,34,41,26,42,28,31,38,2,14,19,37,33,23,43,29,
        15,16,11],
      8,
    ],
  ];

  test('it should return the maximum area of a slice', () => {
    testTuples.forEach((tuple) => {
      expect(maxArea(...tuple)).toBe(tuple[4]);
    });
  });
});

describe('minReorder', () => {
  const testTuples = [
    [
      6,
      [[0,1],[1,3],[2,3],[4,0],[4,5]],
      3,
    ],
    [
      5,
      [[1,0],[1,2],[3,2],[3,4]],
      2,
    ],
    [
      3,
      [[1,0],[2,0]],
      0,
    ],
  ];

  test('it should return the minimum number of changes needed to reorder the list', () => {
    testTuples.forEach(([n, connections, expected]) => {
      expect(minReorder(n, connections)).toBe(expected);
    });
  });
});
