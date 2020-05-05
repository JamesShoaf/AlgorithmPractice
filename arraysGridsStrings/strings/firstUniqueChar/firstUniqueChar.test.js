const {
  firstUniqueChar,
} = require('./firstUniqueChar');

const testTuples = [
  ['', -1],
  ['a', 0],
  ['aa', -1],
  ['abacb', 3],
];

describe('firstUniqueChar', () => {
  test('it should return an integer', () => {
    testTuples.forEach((tuple) => {
      expect(Number.isInteger(firstUniqueChar(tuple[0]))).toBe(true);
    });
  });

  test('it should return the index of the first unique character', () => {
    testTuples.forEach((tuple) => {
      expect(firstUniqueChar(tuple[0])).toBe(tuple[1]);
    });
  });
});
