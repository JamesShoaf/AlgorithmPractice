const {
  rangeBitwiseAnd,
} = require('./rangeBitwiseAnd');

const testArrays = [
  [0, 1, 0],
  [2, 3, 2],
  [5, 7, 4],
  [6, 7, 6],
  [7, 8, 0],
  [10, 11, 10],
];

describe('rangeBitwiseAnd', () => {
  test('it should return an integer', () => {
    testArrays.forEach((array) => {
      expect(Number.isInteger(rangeBitwiseAnd(...array))).toBe(true);
    });
  });

  test('it should return the bitwise AND of the range [m, n]', () => {
    testArrays.forEach((array) => {
      expect(rangeBitwiseAnd(...array)).toBe(array[2]);
    });
  });
});
