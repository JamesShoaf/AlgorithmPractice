const { maxSlidingWindow } = require('./maxSlidingWindow');

describe('maxSlidingWindow', () => {
  test('it should return an empty array if given empty parameters', () => {
    const testTuples = [
      [
        [],
        2,
        [],
      ],
      [
        [1, 2],
        0,
        [],
      ],
      [
        [],
        0,
        [],
      ],
    ];
    testTuples.forEach(([nums, k, expected]) => {
      expect(maxSlidingWindow(nums, k)).toEqual(expected);
    });
  });

  test('it should return a copy of the original array if k = 1', () => {
    const test = [1, 3, 5, -1, -3, -5, 1, 5, 3];
    expect(maxSlidingWindow(test, 1)).toEqual(test);
  });

  test('it handles k = 2', () => {
    const testTuples = [
      [
        [1, 3, 5, -1, -3, -5, 1, 5, 3],
        2,
        [3, 5, 5, -1, -3, 1, 5, 5],
      ],
    ];
    testTuples.forEach(([nums, k, expected]) => {
      expect(maxSlidingWindow(nums, k)).toEqual(expected);
    });
  });

  test('it handles k >= 3', () => {
    const testTuples = [
      [
        [1, 3, 5, -1, -3, -5, 1, 5, 3],
        2,
        [3, 5, 5, -1, -3, 1, 5, 5],
      ],
      [
        [1, 3, 5, -1, -3, -5, 1, 5, 3],
        3,
        [5, 5, 5, -1, 1, 5, 5],
      ],
      [
        [1, 3, 5, -1, -3, -5, 1, 5, 3],
        4,
        [5, 5, 5, 1, 5, 5],
      ],
      [
        [1, 3, 5, -1, -3, -5, 1, 5, 3],
        8,
        [5, 5],
      ],
      [
        [1, 3, -1, -3, 5, 3, 6, 7],
        3,
        [3, 3, 5, 5, 6, 7],
      ],
      [
        [2, 8, 4, 2, 6, 8, 2, 4, 2, 2, 4, 6],
        3,
        [8, 8, 6, 8, 8, 8, 4, 4, 4, 6],
      ],
    ];
    testTuples.forEach(([nums, k, expected]) => {
      expect(maxSlidingWindow(nums, k)).toEqual(expected);
    });
  });

  test('it should return the maximum value from each sliding window of size k', () => {
    const testTuples = [
      [
        [
          -95, 92, -85, 59, -59, -14, 88, -39, 2, 92, 94, 79, 78, -58, 37, 48, 63, -91, 91, 74,
          -28, 39, 90, -9, -72, -88, -72, 93, 38, 14, -83, -2, 21, 4, -75, -65, 3, 63, 100, 59,
          -48, 43, 35, -49, 48, -36, -64, -13, -7, -29, 87, 34, 56, -39, -5, -27, -28, 10, -57,
          100, -43, -98, 19, -59, 78, -28, -91, 67, 41, -64, 76, 5, -58, -89, 83, 26, -7, -82,
          -32, -76, 86, 52, -6, 84, 20, 51, -86, 26, 46, 35, -23, 30, -51, 54, 19, 30, 27, 80,
          45, 22,
        ],
        10,
        [
          92, 94, 94, 94, 94, 94, 94, 94, 94, 94, 94, 91, 91, 91, 91, 91, 91, 91, 93, 93, 93,
          93, 93, 93, 93, 93, 93, 93, 63, 100, 100, 100, 100, 100, 100, 100, 100, 100,
          100, 59, 48, 87, 87, 87, 87, 87, 87, 87, 87, 87, 100, 100, 100, 100, 100, 100,
          100, 100, 100, 100, 78, 78, 78, 78, 78, 83, 83, 83, 83, 83, 83, 86, 86, 86, 86,
          86, 86, 86, 86, 86, 86, 84, 84, 84, 54, 54, 54, 54, 80, 80, 80,
        ],
      ],
    ];
    testTuples.forEach(([nums, k, expected]) => {
      expect(maxSlidingWindow(nums, k)).toEqual(expected);
    });
  });
});
