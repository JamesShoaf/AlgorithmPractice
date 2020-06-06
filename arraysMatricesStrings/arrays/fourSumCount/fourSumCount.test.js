const { fourSumCount } = require('./fourSumCount');

const testTuples = [
  [
    [
      [1, 2],
      [-2, -1],
      [-1, 2],
      [0, 2],
    ],
    2,
  ],
  [
    [
      [1, 1, 1, -1],
      [-2],
      [1, 8],
      [0, 0, 0],
    ],
    9,
  ],
  [
    [
      [],
      [],
      [],
      [],
    ],
    0,
  ],
];

describe('fourSumCount', () => {
  test('it should return the number of combinations that sum to 0', () => {
    testTuples.forEach(([lists, expected]) => {
      expect(fourSumCount(...lists)).toBe(expected);
    });
  });
});
