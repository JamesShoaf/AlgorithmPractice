const { singleNonDuplicate } = require('./singleNonDuplicate');

const testTuples = [
  [
    [1, 1, 2, 3, 3, 4, 4, 8, 8],
    2,
  ],
  [
    [3, 3, 7, 7, 10, 11, 11],
    10,
  ],
  [
    [7],
    7,
  ],
  [
    [1, 22, 22],
    1,
  ],
  [
    [11, 11, 22],
    22,
  ],
];

describe('singleNonDuplicate', () => {
  test('it should return the nonDuplicate', () => {
    testTuples.forEach(([input, output]) => {
      expect(singleNonDuplicate(input)).toBe(output);
    });
  });
});
