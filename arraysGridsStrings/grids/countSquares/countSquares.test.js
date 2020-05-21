const { countSquares } = require('./countSquares');

const testTuples = [
  [
    [], // 0 rows
    0,
  ],
  [
    [
      [], // 1 row, 0 cols
    ],
    0,
  ],
  [
    [
      [0],
    ],
    0,
  ],
  [
    [
      [1],
    ],
    1,
  ],
  [
    [
      [1],
      [1],
    ],
    2,
  ],
  [
    [
      [1, 0],
      [1, 1],
    ],
    3,
  ],
  [
    [
      [1, 1],
      [1, 1],
    ],
    5,
  ],
  [
    [
      [0, 1, 1, 1, 0, 0],
      [1, 1, 1, 1, 1, 1],
      [1, 1, 1, 1, 1, 1],
      [1, 1, 1, 1, 1, 1],
      [1, 1, 1, 1, 1, 1],
      [0, 1, 1, 1, 1, 1],
    ],
    71, // 32 1s, 21 2s, 12 3s, 5 4s, 1 5
  ],
];

describe('countSquares', () => {
  test('it should return the number of squares in the matrix', () => {
    testTuples.forEach(([matrix, expected]) => {
      expect(countSquares(matrix)).toBe(expected);
    });
  });
});
