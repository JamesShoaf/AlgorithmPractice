const {
  jumpGame,
} = require('./jumpGame');

const testArrays = [
  [
    [0, 1],
    false,
  ],
  [
    [0],
    true,
  ],
  [
    [1, 2, 1, 0, 4],
    false,
  ],
  [
    [7, 0, 0, 0, 0, 0, 0, 0],
    true,
  ],
  [
    [3, 0, 0, 0, 0],
    false,
  ],
];

describe('jumpGame', () => {
  test('it should return a boolean', () => {
    testArrays.forEach((testTuple) => {
      expect(typeof jumpGame(testTuple[0])).toBe('boolean');
    });
  });

  test('it should return whether the last index can be reached', () => {
    testArrays.forEach((testTuple) => {
      expect(jumpGame(testTuple[0])).toBe(testTuple[1]);
    });
  });
});
