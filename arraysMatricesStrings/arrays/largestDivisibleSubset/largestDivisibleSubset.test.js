const { largestDivisibleSubset } = require('./largestDivisibleSubset');

describe('largestDivisibleSubset', () => {
  test('it should return the correct subset when only one is possible', () => {
    const testTuples = [
      [
        [1, 2, 4, 8],
        [1, 2, 4, 8],
      ],
      [
        [1, 3, 5, 7, 14],
        [1, 7, 14],
      ],
    ];
    testTuples.forEach(([nums, expected]) => {
      expect(largestDivisibleSubset(nums)).toEqual(expected);
    });
  });

  test('it should return a subset of the longest possible length when multiple are possible', () => {
    const testTuples = [
      [
        [1, 2, 3],
        2,
      ],
      [
        [1, 2, 3, 6, 9],
        3,
      ],
      [
        [1, 2, 3, 4, 5, 6, 7, 14, 28],
        4,
      ],
      [
        [1, 2, 3, 4, 8, 9, 16, 27, 81],
        5,
      ],
    ];
    testTuples.forEach(([nums, expected]) => {
      expect(largestDivisibleSubset(nums).length).toBe(expected);
    });
  });

  test('it should handle a large number of values rapidly', () => {
    const test = [];
    for (let i = 1; i <= 8192; i += 1) test.push(i);
    const expected = [
      1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192,
      // 16384, 10.5s
      // 32768, 45s
    ];
    expect(largestDivisibleSubset(test)).toEqual(expected);
  });
});
