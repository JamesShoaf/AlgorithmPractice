const {
  minimumPathSum,
} = require('./minimumPathSum.js');

describe('minimumPathSum', () => {
  test('it should return an integer', () => {
    expect(Number.isInteger(minimumPathSum([[1]]))).toBe(true);
  });
  test('it should return the correct integer for a path', () => {
    expect(minimumPathSum([
      [1, 3, 1],
      [1, 5, 1],
      [4, 2, 1],
    ])).toBe(7);
  });
});
