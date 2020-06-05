const { spiralOrder } = require('./spiralOrder');

const testTuples = [
  [
    [],
    [],
  ],
  [
    [[]],
    [],
  ],
  [
    [[1]],
    [1],
  ],
  [
    [
      [1, 2, 3],
    ],
    [1, 2, 3],
  ],
  [
    [
      [1],
      [2],
      [3],
    ],
    [1, 2, 3],
  ],
  [
    [
      [1, 2],
      [3, 4],
    ],
    [1, 2, 4, 3],
  ],
  [
    [
      [1, 2, 3],
      [4, 5, 6],
    ],
    [1, 2, 3, 6, 5, 4],
  ],
  [
    [
      [1, 2],
      [3, 4],
      [5, 6],
    ],
    [1, 2, 4, 6, 5, 3],
  ],
  [
    [
      [1, 2, 3],
      [4, 5, 6],
      [7, 8, 9],
    ],
    [1, 2, 3, 6, 9, 8, 7, 4, 5],
  ],
];

describe('spiralOrder', () => {
  test('it should return the values of the array clockwise from the top left', () => {
    testTuples.forEach(([matrix, expected]) => {
      expect(spiralOrder(matrix)).toEqual(expected);
    });
  });
});
