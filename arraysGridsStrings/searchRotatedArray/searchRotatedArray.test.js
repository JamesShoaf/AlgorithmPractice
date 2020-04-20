const {
  searchRotatedArray,
} = require('./searchRotatedArray');

describe('searchRotatedArray', () => {
  const pivots = [
    [0],
    [1, 0],
    [3, 4, 5, 0, 2],
    [5, 6, 7, 8, 0, 3, 4],
    [8, 9, 2, 3, 4],
    [4, 5, 6, 7, 8, 1, 2, 3],
    [5, 1, 2, 3, 4],
  ];

  test('it should return -1 if the searched element is not in the array', () => {
    pivots.forEach((array) => expect(searchRotatedArray(array, 20)).toBe(-1));
  });

  test('it should return the index of the element', () => {
    expect(searchRotatedArray(pivots[0], 0)).toBe(0);
    expect(searchRotatedArray(pivots[1], 0)).toBe(1);
    expect(searchRotatedArray(pivots[2], 0)).toBe(3);
    expect(searchRotatedArray(pivots[3], 0)).toBe(4);
    expect(searchRotatedArray(pivots[4], 9)).toBe(1);
    expect(searchRotatedArray(pivots[5], 8)).toBe(4);
    expect(searchRotatedArray(pivots[6], 1)).toBe(1);
  });
});
