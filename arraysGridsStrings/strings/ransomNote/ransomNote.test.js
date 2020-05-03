const {
  ransomNote,
} = require('./ransomNote');

describe('ransomNote', () => {
  const testTuples = [
    [
      'a',
      'b',
      false,
    ],
    [
      'aa',
      'ab',
      false,
    ],
    [
      'aa',
      'aab',
      true,
    ],
  ];

  test('it returns a boolean', () => {
    testTuples.forEach((tuple) => {
      expect(typeof ransomNote(...tuple)).toBe('boolean');
    });
  });

  test('it returns the correct answer', () => {
    testTuples.forEach((tuple) => {
      expect(ransomNote(...tuple)).toBe(tuple[2]);
    });
  });
});
