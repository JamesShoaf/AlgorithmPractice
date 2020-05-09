const {
  MakeChange,
} = require('./makeChange');

const denominations = [
  [1],
  [1, 2, 3],
  [1, 5, 10, 25, 100, 200, 500, 1000],
  [2, 5],
];

const testSums = [0, 1, 2, 3, 4, 5, 10];

const waysToMakeChange = [
  [1, 1, 1, 1, 1, 1, 1],
  [1, 1, 2, 3, 4, 5, 14],
  [1, 1, 1, 1, 1, 2, 4],
  [1, 0, 1, 0, 1, 1, 2],
];

describe('makeChange', () => {
  const changeMakers = denominations.map((denom) => new MakeChange(denom));
  test('it should return an integer', () => {
    changeMakers.forEach((changer) => {
      testSums.forEach((sum) => {
        const received = changer.makeChange(sum);
        expect(Number.isInteger(received)).toBe(true);
      });
    });
  });

  test('it should return the correct number of ways to make change', () => {
    changeMakers.forEach((changer, row) => {
      testSums.forEach((sum, col) => {
        const expected = waysToMakeChange[row][col];
        const received = changer.makeChange(sum);
        expect(received).toBe(expected);
      });
    });
  });
});
