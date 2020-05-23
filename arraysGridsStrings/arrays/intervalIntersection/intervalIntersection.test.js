const { intervalIntersection } = require('./intervalIntersection');

const testTuples = [
  [
    [[0, 2], [5, 10], [13, 23], [24, 25]],
    [[1, 5], [8, 12], [15, 24], [25, 26]],
    [[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]],
  ],
  [
    [[1, 5]],
    [[6, 10]],
    [],
  ],
  [
    [[14, 16]],
    [[7, 13], [16, 20]],
    [[16, 16]],
  ],
  [
    [[1.1, 2.2], [3.3, 5.5]],
    [[1.2, 2.4], [3.1, 3.7], [3.9, 5.2], [5.3, 6.1]],
    [[1.2, 2.2], [3.3, 3.7], [3.9, 5.2], [5.3, 5.5]],
  ],
  [
    [[4, 5], [8, 11], [13, 15], [19, 20]],
    [[0, 1], [3, 6], [16, 17], [18, 19]],
    [[4, 5], [19, 19]],
  ],
];

describe('intervalIntersections', () => {
  test('it should return the correct interval', () => {
    testTuples.forEach(([intA, intB, expected]) => {
      expect(intervalIntersection(intA, intB)).toEqual(expected);
    });
  });
});
