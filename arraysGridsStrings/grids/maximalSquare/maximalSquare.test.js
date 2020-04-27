const {
  maximalSquare,
} = require('./maximalSquare');

const testArrays = [
  [
    [
      [],
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
      [1, 0, 1, 0, 0],
      [1, 0, 1, 1, 1],
      [1, 1, 1, 1, 1],
      [1, 0, 0, 1, 0],
    ],
    4,
  ],
  [
    [
      [1, 0, 1, 0, 0],
      [1, 0, 1, 1, 1],
      [1, 1, 1, 1, 1],
      [1, 0, 1, 1, 1],
    ],
    9,
  ],
  [
    [
      [1, 1, 1, 1, 0],
      [1, 1, 1, 1, 1],
      [1, 1, 1, 1, 1],
      [1, 1, 1, 1, 1],
    ],
    16,
  ],
];

describe('maximalSquare', () => {
  test('it should return an integer', () => {
    testArrays.forEach((testTuple) => {
      expect(Number.isInteger(maximalSquare(testTuple[0]))).toBe(true);
    });
  });

  test('it should return the area of the largest square', () => {
    testArrays.forEach((testTuple) => {
      expect(maximalSquare(testTuple[0])).toBe(testTuple[1]);
    });
  });
});
