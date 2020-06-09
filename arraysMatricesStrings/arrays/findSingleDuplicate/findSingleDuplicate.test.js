const { findSingleDuplicate } = require('./findSingleDuplicate');

const testTuples = [
  [
    [1, 3, 4, 2, 2],
    2,
  ],
  [
    [3, 1, 3, 4, 2],
    3,
  ],
];

describe('findSingleDuplicate', () => {
  test('it should find the single duplicate integer', () => {
    testTuples.forEach(([nums, expected]) => {
      expect(findSingleDuplicate(nums)).toBe(expected);
    });
  });
});
