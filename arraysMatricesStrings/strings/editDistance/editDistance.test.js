const { editDistance } = require('./editDistance');

const testTuples = [
  [
    'horse',
    'ros',
    3,
  ],
  [
    'intention',
    'execution',
    5,
  ],
  [
    '',
    'just',
    4,
  ],
  [
    'aabbaa',
    'abaa',
    2,
  ],
  [
    '',
    '',
    0,
  ],
];

describe('editDistance', () => {
  test('it should return the edit distance between the strings', () => {
    testTuples.forEach(([word1, word2, expected]) => {
      expect(editDistance(word1, word2)).toBe(expected);
    });
  });
});
