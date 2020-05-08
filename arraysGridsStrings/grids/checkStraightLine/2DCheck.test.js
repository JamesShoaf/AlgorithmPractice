const {
  checkStraightLine,
} = require('./2DCheck');

const testTuples = [
  [
    [
      [1, 2],
      [2, 3],
      [3, 4],
      [4, 5],
      [5, 6],
      [6, 7],
    ],
    true,
  ],
  [[[-1, -1]], true],
  [
    [
      [1, 0],
      [2, 0],
      [3, 0],
      [4, 0],
    ],
    true,
  ],
  [
    [
      [0, 1],
      [0, 2],
      [0, 3],
      [0, 4],
    ],
    true,
  ],
  [
    [
      [1, 0],
      [2, 0],
      [3, 0],
      [4, 1],
    ],
    false,
  ],
  [
    [
      [0, 1],
      [0, 2],
      [0, 3],
      [1, 4],
    ],
    false,
  ],
];

describe('2D checkStraightLine', () => {
  test('it should return a boolean', () => {
    testTuples.forEach((tuple) => {
      expect(typeof checkStraightLine(...tuple)).toBe('boolean');
    });
  });
  test('it should return whether the coordinates are in a straight line', () => {
    testTuples.forEach((tuple) => {
      expect(checkStraightLine(...tuple)).toBe(tuple[1]);
    });
  });
});
