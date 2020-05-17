const { findAnagrams } = require('./findAnagrams');

const testTuples = [
  [
    'cbaebabacd',
    'abc',
    [0, 6],
  ],
  [
    'abab',
    'ab',
    [0, 1, 2],
  ],
];

describe('findAnagrams', () => {
  test('it should return an array of indices in the first string that begin anagrams of the second', () => {
    testTuples.forEach(([string, permute, expected]) => {
      expect(findAnagrams(string, permute)).toEqual(expected);
    });
  });
});
