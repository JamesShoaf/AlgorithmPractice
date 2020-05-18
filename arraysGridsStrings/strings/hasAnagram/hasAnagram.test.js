const { hasAnagram } = require('./hasAnagram');

const testTuples = [
  ['ab', 'eidbaooo', true],
  ['ab', 'eidboaoo', false],
  ['', 'eidbaooo', true],
  ['not', 'yes', false],
  ['adc', 'dca', true],
];

describe('hasAnagram', () => {
  test('it should return whether the second string contains any substrings that are anagrams of the first', () => {
    testTuples.forEach(([s1, s2, expected]) => {
      expect(hasAnagram(s1, s2)).toBe(expected);
    });
  });
});
