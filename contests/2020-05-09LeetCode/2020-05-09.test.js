const { buildArray } = require('./buildArray');

const { countTriplets } = require('./countTriplets');

const { minTime } = require('./minTime');

describe('buildArray', () => {
  const testTuples = [
    [[1, 3], 3, ['Push', 'Push', 'Pop', 'Push']],
    [[1, 2, 3], 3, ['Push', 'Push', 'Push']],
    [[1, 2], 3, ['Push', 'Push']],
    [[2, 3, 4], 3, ['Push', 'Pop', 'Push', 'Push', 'Push']],
  ];
  test('it should return the correct output', () => {
    testTuples.forEach((tuple) => {
      expect(buildArray(...tuple)).toEqual(tuple[2]);
    });
  });
});

describe('countTriplets', () => {
  const testTuples = [
    [[2, 3, 1, 6, 7], 4],
    [[1, 1, 1, 1, 1], 10],
    [[2, 3], 0],
    [[1, 3, 5, 7, 9], 3],
    [[7, 11, 12, 9, 5, 2, 7, 17, 22], 8],
  ];
  test('it should return the correct output', () => {
    testTuples.forEach((tuple) => {
      expect(countTriplets(...tuple)).toBe(tuple[1]);
    });
  });
});

describe('minPath', () => {
  const testTuples = [
    [
      7,
      [
        [0, 1],
        [0, 2],
        [1, 4],
        [1, 5],
        [2, 3],
        [2, 6],
      ],
      [false, false, true, false, true, true, false],
      8,
    ],
    [
      7,
      [
        [0, 1],
        [0, 2],
        [1, 4],
        [1, 5],
        [2, 3],
        [2, 6],
      ],
      [false, false, true, false, false, true, false],
      6,
    ],
    [
      7,
      [
        [0, 1],
        [0, 2],
        [1, 4],
        [1, 5],
        [2, 3],
        [2, 6],
      ],
      [false, false, false, false, false, false, false],
      0,
    ],
  ];
  test('it should return the minimum path length', () => {
    testTuples.forEach((tuple) => expect(minTime(...tuple)).toBe(tuple[3]));
  });
});
