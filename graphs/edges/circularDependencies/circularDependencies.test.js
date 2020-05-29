const { circularDependencies } = require('./circularDependencies');

const testTuples = [
  [2, [[1, 0]], true],
  [
    2,
    [
      [1, 0],
      [0, 1],
    ],
    false,
  ],
  [
    3,
    [
      [0, 1],
      [1, 2],
      [2, 0],
    ],
    false,
  ],
  [
    4,
    [
      [0, 1],
      [3, 1],
      [1, 3],
      [3, 2],
    ],
    false,
  ],
  [
    4,
    [
      [0, 1],
      [0, 2],
      [1, 3],
      [2, 3],
    ],
    true,
  ],
  [
    4,
    [
      [0, 1],
      [1, 2],
      [2, 3],
      [3, 0],
    ],
    false,
  ],
  [
    1,
    [
      [0, 0],
    ],
    false,
  ],
];

describe('circularDependencies', () => {
  test('it should return true if there are no circular dependencies', () => {
    testTuples.forEach(([N, dependencies, expected]) => {
      expect(circularDependencies(N, dependencies)).toBe(expected);
    });
  });
});
