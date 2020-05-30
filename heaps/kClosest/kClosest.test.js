const { kClosest } = require('./kClosest');

const testTuples = [
  [
    [
      [1, 3],
      [-2, 2],
    ],
    1,
    [[-2, 2]],
  ],
  [
    [
      [3, 3],
      [5, -1],
      [-2, 4],
    ],
    2,
    [
      [3, 3],
      [-2, 4],
    ],
  ],
];

describe('kClosest', () => {
  test('it should return the k closest points to (0, 0)', () => {
    testTuples.forEach(([points, k, expected]) => {
      expect(kClosest(points, k)).toEqual(expected);
    });
  });
});
