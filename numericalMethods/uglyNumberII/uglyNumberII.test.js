const { nthUglyNumber } = require('./uglyNumberII');

describe('nthUglyNumber', () => {
  test('it should return -1 if provided invalid input', () => {
    const invalidInputs = [2.5, -1, 0, 'a', true, false, null, undefined, NaN, Symbol(1)];
    invalidInputs.forEach((input) => {
      expect(nthUglyNumber(input)).toBe(-1);
    });
  });

  test('it should return the nth ugly number', () => {
    const testTuples = [
      [1, 1],
      [2, 2],
      [3, 3],
      [4, 4],
      [5, 5],
      [6, 6],
      [7, 8],
      [8, 9],
      [9, 10],
      [10, 12],
      [11, 15],
      [12, 16],
      [13, 18],
      [14, 20],
      [15, 24],
      [1690, 2123366400],
    ];
    testTuples.forEach(([n, expected]) => {
      expect(nthUglyNumber(n)).toBe(expected);
    });
  });
});
