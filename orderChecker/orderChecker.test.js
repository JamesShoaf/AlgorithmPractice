const orderChecker = require('./orderChecker');

test('it should return true if passed three empty arrays', () => {
  expect(orderChecker([], [], [])).toBe(true);
});

test('it should return false if the third array does not contain all elements of the first two arrays', () => {
  expect(orderChecker([1], [2], [1])).toBe(false);
  expect(orderChecker([1], [2], [1, 1])).toBe(false);
  expect(orderChecker([1], [2], [2, 2])).toBe(false);
});
