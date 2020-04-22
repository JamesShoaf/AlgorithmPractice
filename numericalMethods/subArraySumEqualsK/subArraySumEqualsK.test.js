const {
  subArraySumEqualsK,
} = require('./subArraySumEqualsK');

const testArray = [1, -1, 0, 1, 1, -1, 0, -1];

describe('subArraySumEqualsK', () => {
  test('it returns an integer', () => {
    for (let i = -2; i < 4; i += 1) {
      expect(typeof subArraySumEqualsK(testArray, i)).toBe('number');
    }
  });

  test('it returns the number of continuous subArrays', () => {
    expect(subArraySumEqualsK(testArray, -2)).toBe(1);
    expect(subArraySumEqualsK(testArray, -1)).toBe(8);
    expect(subArraySumEqualsK(testArray, 0)).toBe(12);
    expect(subArraySumEqualsK(testArray, 1)).toBe(12);
    expect(subArraySumEqualsK(testArray, 2)).toBe(3);
    expect(subArraySumEqualsK(testArray, 3)).toBe(0);
  });
});
