const { longestConsecutiveSequence } = require('./longestConsecutiveSequence');

const testTuples = [
  [
    [],
    0,
  ],
  [
    [50],
    1,
  ],
  [
    [100, 4, 200, 1, 3, 2],
    4,
  ],
  [
    [1073741824, 0, 3, 2],
    2,
  ],
];

describe('longestConsecutiveSequence', () => {
  test('it should return the length of the longest consecutive sequence of integers', () => {
    testTuples.forEach(([nums, expected]) => {
      expect(longestConsecutiveSequence(nums)).toBe(expected);
    });
  });
});
