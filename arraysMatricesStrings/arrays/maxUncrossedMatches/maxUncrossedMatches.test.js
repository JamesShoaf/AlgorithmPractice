const { maxUncrossedMatches } = require('./maxUncrossedMatches');

const testTuples = [
  [
    [1, 4, 2],
    [1, 2, 4],
    2,
  ],
  [
    [2, 5, 1, 2, 5],
    [10, 5, 2, 1, 5, 2],
    3,
  ],
  [
    [1, 3, 7, 1, 7, 5],
    [1, 9, 2, 5, 1],
    2,
  ],
  [
    [1, 3, 5, 7, 2, 4, 6, 1, 3, 5, 2, 4, 1],
    [1, 2, 3, 4, 5, 6, 7],
    4,
  ],
  [
    [6, 5, 7, 2, 4, 6, 1, 3, 5, 7],
    [1, 2, 3, 4, 5, 6, 7],
    4,
  ],
  [
    [1, 2, 4, 1, 4, 4, 3, 5, 5, 1, 4, 4, 4, 1, 4, 3, 4, 2, 4, 2],
    [2, 4, 1, 1, 3, 5, 2, 1, 5, 1, 2, 3, 3, 2, 1, 4, 1, 2, 5, 5],
    11,
  ],
];

describe('maxUncrossedMatches', () => {
  test('it should return the correct number of matches', () => {
    testTuples.forEach(([a, b, expected]) => {
      expect(maxUncrossedMatches(a, b)).toBe(expected);
    });
  });
});
