const { maxAreaBetweenIndexes } = require('./maxAreaBetweenIndexes');

const testTuples = [
  [
    [8, 8],
    8,
  ],
  [
    [1, 8, 6, 100, 100, 4, 3, 7],
    100,
  ],
  [
    [1, 8, 6, 2, 5, 4, 8, 3, 7],
    49,
  ],
  [
    [1, 8, 6, 2, 5, 4, 8, 3, 7, 1],
    49,
  ],
];

describe('maxAreaBetweenIndexes', () => {
  test('it should return the maximum area between indexes', () => {
    testTuples.forEach(([heights, expected]) => {
      expect(maxAreaBetweenIndexes(heights)).toBe(expected);
    });
  });
});
