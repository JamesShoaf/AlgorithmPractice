const {
  buildArray,
} = require('./buildArray');

describe('buildArray', () => {
  const testTuples = [
    [
      [1, 3],
      3,
      ['Push', 'Push', 'Pop', 'Push'],
    ],
    [
      [1, 2, 3],
      3,
      ['Push', 'Push', 'Push'],
    ],
    [
      [1, 2],
      3,
      ['Push', 'Push'],
    ],
    [
      [2, 3, 4],
      3,
      ['Push', 'Pop', 'Push', 'Push', 'Push'],
    ],
  ];
  test('it should return the correct output', () => {
    testTuples.forEach((tuple) => {
      expect(buildArray(...tuple)).toEqual(tuple[2]);
    });
  });
});
