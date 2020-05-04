const {
  numberComplement,
} = require('./numberComplement');

describe('numberComplement', () => {
  const testTuples = [
    [0, 1],
    [1, 0],
    [2, 1],
    [3, 0],
    [4, 3],
    [5, 2],
    [6, 1],
    [7, 0],
    [8, 7],
    [15, 0],
    [16, 15],
    [31, 0],
  ];

  test('it should return an integer', () => {
    testTuples.forEach((tuple) => {
      expect(Number.isInteger(numberComplement(tuple[0]))).toBe(true);
    });
  });

  test('it should return the complement', () => {
    testTuples.forEach((tuple) => {
      expect(numberComplement(tuple[0])).toBe(tuple[1]);
    });
  });
});
