const { sortByFrequency } = require('./sortByFrequency');

const testTuples = [
  [
    'tree',
    new Set(['eert', 'eetr']),
  ],
  [
    'cccaaa',
    new Set(['aaaccc', 'cccaaa']),
  ],
  [
    'Aabb',
    new Set(['bbAa', 'bbaA']),
  ],
  ['banana', new Set(['aaannb'])],
  ['', new Set([''])],
];

describe('sortByFrequency', () => {
  test('it should return a string with characters sorted by frequency', () => {
    testTuples.forEach(([input, acceptableAnswers]) => {
      const output = sortByFrequency(input);
      expect(acceptableAnswers.has(output)).toBe(true);
    });
  });
});
