const { possibleBipartition } = require('./possibleBipartition');

const testTuples = [
  [
    4,
    [
      [1, 2],
      [1, 3],
      [2, 4],
    ],
    true,
  ],
  [
    3,
    [
      [1, 2],
      [1, 3],
      [2, 3],
    ],
    false,
  ],
  [
    5,
    [
      [1, 2],
      [2, 3],
      [3, 4],
      [4, 5],
      [1, 5],
    ],
    false,
  ],
];

describe('possibleBipartition', () => {
  test('it should return whether bipartition is possible', () => {
    testTuples.forEach(([N, crushes, expected]) => {
      expect(possibleBipartition(N, crushes)).toBe(expected);
    });
  });
});
