const { sortColors } = require('./sortColors');

const testTuples = [
  [
    [2, 0, 2, 1, 1, 0],
    [0, 0, 1, 1, 2, 2],
  ],
  [
    [2, 2, 1, 1, 0, 0, 1, 2, 0],
    [0, 0, 0, 1, 1, 1, 2, 2, 2],
  ],
  [
    [1, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 1],
  ],
  [
    [2, 2, 2, 0, 0, 0],
    [0, 0, 0, 2, 2, 2],
  ],
  [
    [2, 2, 2, 1, 1, 1],
    [1, 1, 1, 2, 2, 2],
  ],
];

describe('sortColors', () => {
  test('it should sort the array in place', () => {
    testTuples.forEach(([nums, expected]) => {
      sortColors(nums);
      expect(nums).toEqual(expected);
    });
  });
});
