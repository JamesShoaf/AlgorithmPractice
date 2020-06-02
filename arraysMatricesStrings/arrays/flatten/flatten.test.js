const { flatten } = require('./flatten');

const testTuples = [
  [
    [1, 2, 3, [4, 5]],
    [1, 2, 3, 4, 5],
  ],
  [
    [1, [2, [3, 4], [[5]]]],
    [1, 2, 3, 4, 5],
  ],
  [
    [[1], [2], [3]],
    [1, 2, 3],
  ],
  [
    [[[[1], [[[2]]], [[[[[[[3]]]]]]]]]],
    [1, 2, 3],
  ],
];

describe('flatten', () => {
  test('it should return a flat array', () => {
    testTuples.forEach(([input, expected]) => {
      expect(flatten(input)).toEqual(expected);
    });
  });
});
