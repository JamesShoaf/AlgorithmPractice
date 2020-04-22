const contiguousArray = require('./contiguousArray');

describe('contiguousArray', () => {
  const testArrays = {
    0: [],
    1: [0, 1],
    2: [0, 1, 1, 0],
    3: [0, 1, 1, 1],
    4: [1, 1, 1, 1, 1, 0, 0, 0],
    foo: ['foo', 'bar'],
  };

  test('it should return an integer', () => {
    expect(Number.isInteger(contiguousArray(testArrays[0]))).toBe(true);
    expect(Number.isInteger(contiguousArray(testArrays[1]))).toBe(true);
    expect(Number.isInteger(contiguousArray(testArrays[2]))).toBe(true);
    expect(Number.isInteger(contiguousArray(testArrays[3]))).toBe(true);
    expect(Number.isInteger(contiguousArray(testArrays[4]))).toBe(true);
    expect(Number.isInteger(contiguousArray(testArrays.foo))).toBe(true);
  });

  test('it should return the length of the longest array containing an equal number of 1s and 0s', () => {
    expect(contiguousArray(testArrays[0])).toBe(0);
    expect(contiguousArray(testArrays[1])).toBe(2);
    expect(contiguousArray(testArrays[2])).toBe(4);
    expect(contiguousArray(testArrays[3])).toBe(2);
    expect(contiguousArray(testArrays[4])).toBe(6);
    expect(contiguousArray(testArrays.foo)).toBe(0);
  });
});
