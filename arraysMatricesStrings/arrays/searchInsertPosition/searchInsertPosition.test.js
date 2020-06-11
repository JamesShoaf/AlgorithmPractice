const { searchInsertPosition } = require('./searchInsertPosition');

const testTuples = [
  [
    [1, 3, 5, 6],
    5,
    2,
  ],
  [
    [1, 3, 5, 6],
    2,
    1,
  ],
  [
    [1, 3, 5, 6],
    7,
    4,
  ],
  [
    [1, 3, 5, 6],
    0,
    0,
  ],
  [
    [1, 3, 5, 6, 8],
    0,
    0,
  ],
  [
    [1, 3, 5, 6, 8],
    2,
    1,
  ],
  [
    [1, 3, 5, 6, 8],
    4,
    2,
  ],
  [
    [1, 3, 5, 6, 8],
    5.5,
    3,
  ],
  [
    [1, 3, 5, 6, 8],
    7,
    4,
  ],
  [
    [1, 3, 5, 6, 8],
    9,
    5,
  ],
  [
    [],
    0,
    0,
  ],
];

describe('searchInsertPosition', () => {
  test('it should return the index at which the target is found or would be inserted in order', () => {
    testTuples.forEach(([nums, target, expected]) => {
      expect(searchInsertPosition(nums, target)).toBe(expected);
    });
  });
});
