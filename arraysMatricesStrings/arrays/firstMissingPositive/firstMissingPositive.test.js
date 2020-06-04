const { firstMissingPositive } = require('./firstMissingPositive');

const testTuples = [
  [
    [],
    1,
  ],
  [
    [7, 8, 9, 11, 12],
    1,
  ],
  [
    [3, 4, -1, 1],
    2,
  ],
  [
    [1, 2, 0],
    3,
  ],
];

describe('firstMissingPositive', () => {
  test('it should return the correct integer', () => {
    testTuples.forEach(([nums, expected]) => {
      expect(firstMissingPositive(nums)).toBe(expected);
    });
  });
});
