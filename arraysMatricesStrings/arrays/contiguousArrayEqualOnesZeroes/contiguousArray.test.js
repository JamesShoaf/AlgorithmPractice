const contiguousArray = require('./contiguousArray');
const { challengeArray } = require('./challengeArray');

describe('contiguousArray', () => {
  const testTuples = [
    [
      [],
      0,
    ],
    [
      [0, 1],
      2,
    ],
    [
      [0, 1, 1, 0],
      4,
    ],
    [
      [0, 1, 1, 1],
      2,
    ],
    [
      [1, 1, 1, 1, 1, 0, 0, 0],
      6,
    ],
    [
      ['foo', 'bar'],
      0,
    ],
    [
      challengeArray,
      26024,
    ],
  ];

  test('it should return an integer', () => {
    testTuples.forEach(([array]) => {
      expect(Number.isInteger(contiguousArray(array))).toBe(true);
    });
  });

  test('it should return the length of the longest array containing an equal number of 1s and 0s', () => {
    testTuples.forEach(([array, expected]) => {
      expect(contiguousArray(array)).toBe(expected);
    });
  });
});
