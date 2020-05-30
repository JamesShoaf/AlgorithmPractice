const {
  getGeneration,
} = require('./getGeneration');

const testTuples = [
  [
    [
      [0],
    ],
    1,
    [
      [0],
    ],
  ],
  [
    [
      [1],
    ],
    1,
    [
      [0],
    ],
  ],
  [
    [
      [1],
    ],
    0,
    [
      [1],
    ],
  ],
  [
    [
      [1, 1, 0],
      [1, 1, 0],
      [0, 0, 0],
    ],
    1,
    [
      [1, 1, 0],
      [1, 1, 0],
      [0, 0, 0],
    ],
  ],
  [
    [
      [0, 1, 0],
      [0, 1, 0],
      [0, 1, 0],
    ],
    1,
    [
      [0, 0, 0],
      [1, 1, 1],
      [0, 0, 0],
    ],
  ],
  [
    [
      [0, 1, 0],
      [0, 1, 0],
      [0, 1, 0],
    ],
    2,
    [
      [0, 1, 0],
      [0, 1, 0],
      [0, 1, 0],
    ],
  ],
];

describe('getGeneration', () => {
  test('it should return an array with the correct dimensions', () => {
    testTuples.forEach((tuple) => {
      const result = getGeneration(...tuple);
      expect(Array.isArray(result)).toBe(true);
      expect(result.length).toBe(tuple[2].length);
      expect(result[0].length).toBe(tuple[2][0].length);
    });
  });

  test('it should return the correct Array', () => {
    testTuples.forEach((tuple) => {
      expect(getGeneration(...tuple)).toEqual(tuple[2]);
    });
  });
});
