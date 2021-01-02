const { getLastMoment } = require('./getLastMoment');

describe('getLastMoment', () => {
  test('it should return the correct answer', () => {
    const testTuples = [
      [
        [
          4,
          [4, 3],
          [0, 1],
        ],
        4,
      ],
      [
        [
          7,
          [0, 1, 2, 3, 4, 5, 6, 7],
          [],
        ],
        7,
      ],
      [
        [
          9,
          [5],
          [4],
        ],
        5,
      ],
      [
        [
          6,
          [6],
          [0],
        ],
        6,
      ],
      [
        [
          1,
          [0],
          [1],
        ],
        0,
      ],
    ];
    testTuples.forEach(([inputs, expected]) => {
      expect(getLastMoment(...inputs)).toBe(expected);
    });
  });
});
