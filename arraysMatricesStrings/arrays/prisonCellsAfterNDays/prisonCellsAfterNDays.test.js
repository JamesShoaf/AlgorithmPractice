const { prisonCellsAfterNDays } = require('./prisonCellsAfterNDays');

const testTuples = [
  [
    [
      [1, 1, 1],
      0,
    ],
    [1, 1, 1],
  ],
  [
    [
      [1, 1, 1],
      1,
    ],
    [0, 1, 0],
  ],
  [
    [
      [1, 1, 1],
      2,
    ],
    [0, 1, 0],
  ],
  [
    [
      [1, 1, 1],
      100,
    ],
    [0, 1, 0],
  ],
  [
    [
      [1, 1, 1, 1],
      0,
    ],
    [1, 1, 1, 1],
  ],
  [
    [
      [1, 1, 1, 1],
      1,
    ],
    [0, 1, 1, 0],
  ],
  [
    [
      [1, 1, 1, 1],
      2,
    ],
    [0, 0, 0, 0],
  ],
  [
    [
      [1, 1, 1, 1],
      3,
    ],
    [0, 1, 1, 0],
  ],
  [
    [
      [1, 1, 1, 1],
      4,
    ],
    [0, 0, 0, 0],
  ],
  [
    [
      [1, 1, 1, 1],
      5,
    ],
    [0, 1, 1, 0],
  ],
  [
    [
      [1, 1, 1, 0],
      0,
    ],
    [1, 1, 1, 0],
  ],
  [
    [
      [1, 1, 1, 0],
      1,
    ],
    [0, 1, 0, 0],
  ],
  [
    [
      [1, 1, 1, 0],
      2,
    ],
    [0, 1, 0, 0],
  ],
  [
    [
      [1, 1, 1, 0],
      100,
    ],
    [0, 1, 0, 0],
  ],
  [
    [
      [0, 0, 0, 1],
      0,
    ],
    [0, 0, 0, 1],
  ],
  [
    [
      [0, 0, 0, 1],
      1,
    ],
    [0, 1, 0, 0],
  ],
  [
    [
      [0, 0, 0, 1],
      100,
    ],
    [0, 1, 0, 0],
  ],
];

describe('prisonCellsAfterNDays', () => {
  test('it should return an empty array if passed an empty array', () => {
    expect(prisonCellsAfterNDays([], 0)).toEqual([]);
    expect(prisonCellsAfterNDays([], 100)).toEqual([]);
  });

  test('it should return the correct array', () => {
    testTuples.forEach(([inputs, expected]) => {
      expect(prisonCellsAfterNDays(...inputs)).toEqual(expected);
    });
  });
});
