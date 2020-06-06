const { reconstructHeightQueue } = require('./reconstructHeightQueue');

const testTuples = [
  [
    [
      [7, 0],
      [4, 4],
      [7, 1],
      [5, 0],
      [6, 1],
      [5, 2],
    ],
    [
      [5, 0],
      [7, 0],
      [5, 2],
      [6, 1],
      [4, 4],
      [7, 1],
    ],
  ],
];

describe('reconstructHeightQueue', () => {
  test('it should return the queue in order', () => {
    testTuples.forEach(([people, expected]) => {
      expect(reconstructHeightQueue(people)).toEqual(expected);
    });
  });
});
