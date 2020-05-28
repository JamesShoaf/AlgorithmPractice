const { countBits } = require('./countBits');

const testTuples = [
  [
    0,
    [0],
  ],
  [
    1,
    [0, 1],
  ],
  [
    2,
    [0, 1, 1],
  ],
  [
    5,
    [0, 1, 1, 2, 1, 2],
  ],
  [
    15,
    [
      0,
      1,
      1, 2,
      1, 2, 2, 3,
      1, 2, 2, 3, 2, 3, 3, 4,
    ],
  ],
];

describe('countBits', () => {
  test('it should return the correct array', () => {
    testTuples.forEach(([num, expected]) => {
      expect(countBits(num)).toEqual(expected);
    });
  });
});
