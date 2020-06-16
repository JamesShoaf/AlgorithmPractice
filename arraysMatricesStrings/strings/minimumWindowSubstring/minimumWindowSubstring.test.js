const { minimumWindowSubstring } = require('./minimumWindowSubstring');

const testTuples = [
  [
    'ADOBECODEBANC',
    'ABC',
    'BANC',
  ],
  [
    'ADOBECODEBANC',
    'ABBC',
    'BECODEBA',
  ],
  [
    'ADOBECODEBANC',
    'DC',
    'COD',
  ],
  [
    'ABBBBBBBABBBBBABBBABBABAA',
    'AA',
    'AA',
  ],
  [
    'AB',
    'B',
    'B',
  ],
  [
    'BBA',
    'AB',
    'BA',
  ],
  [
    'BBAA',
    'AB',
    'BA',
  ],
  [
    'BB',
    'B',
    'B',
  ],
  [
    'B',
    'B',
    'B',
  ],
];

describe('minimumWindowSubstring', () => {
  test('it should return a string representing the minimum window', () => {
    testTuples.forEach(([s, t, expected]) => {
      expect(minimumWindowSubstring(s, t)).toBe(expected);
    });
  });

  test('it should return an empty string if no window exists', () => {
    expect(minimumWindowSubstring('ADOBECODEBANC', 'X')).toBe('');
    expect(minimumWindowSubstring('ADOBECODEBANC', 'AG')).toBe('');
    expect(minimumWindowSubstring('', 'B')).toBe('');
  });
});
