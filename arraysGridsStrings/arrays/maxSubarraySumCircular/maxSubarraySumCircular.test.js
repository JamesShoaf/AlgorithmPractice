const { maxSubarraySumCircular } = require('./maxSubarraySumCircular');

const testTuples = [
  [
    [1, 5, -2, 3, -1, 5, 1],
    14,
  ],
  [
    [1, 5, -2, -50, 3, -1, 5, 1],
    14,
  ],
  [
    [1, 5, -2, -50, 13, -13, 3, -1, 5, 1],
    14,
  ],
  [
    [1, -2, 3, -2],
    3,
  ],
  [
    [5, -3, 5],
    10,
  ],
  [
    [3, -1, 2, -1],
    4,
  ],
  [
    [3, -2, 2, -3],
    3,
  ],
  [
    [-2, -3, -1],
    -1,
  ],
];

describe('maxSubarraySumCircular', () => {
  test('it should return the correct sum', () => {
    testTuples.forEach(([nums, expected]) => {
      expect(maxSubarraySumCircular(nums)).toBe(expected);
    });
  });
});
