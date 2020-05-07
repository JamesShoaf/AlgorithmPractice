const {
  fibonacci,
} = require('./fibonacci');

const testTuples = [
  [0, 0],
  [1, 1],
  [2, 1],
  [3, 2],
  [4, 3],
  [5, 5],
  [6, 8],
  [7, 13],
  [8, 21],
  [9, 34],
  [50, 12586269025],
  [78, 8944394323791464],
];

describe('fibonacci', () => {
  test('it should return an integer', () => {
    testTuples.forEach((tuple) => {
      expect(Number.isInteger(fibonacci(...tuple))).toBe(true);
    });
  });

  test('it should return the correct number', () => {
    testTuples.forEach((tuple) => {
      expect(fibonacci(...tuple)).toBe(tuple[1]);
    });
  });
});
