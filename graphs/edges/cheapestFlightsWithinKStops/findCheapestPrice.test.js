const { findCheapestPrice } = require('./findCheapestPrice');

// n, flights, src, dst, k

const testTuples = [
  [
    [
      3,
      [
        [0, 1, 100],
        [1, 2, 100],
        [0, 2, 500],
      ],
      0,
      2,
      0,
    ],
    500,
  ],
  [
    [
      3,
      [
        [0, 1, 100],
        [1, 2, 100],
        [0, 2, 500],
      ],
      0,
      2,
      1,
    ],
    200,
  ],
  [
    [
      3,
      [
        [0, 1, 100],
        [1, 2, 100],
      ],
      0,
      2,
      0,
    ],
    -1,
  ],
  [
    [
      5,
      [
        [0, 1, 5],
        [1, 2, 5],
        [0, 3, 2],
        [3, 1, 2],
        [1, 4, 1],
        [4, 2, 1],
      ],
      0,
      2,
      0,
    ],
    -1,
  ],
  [
    [
      5,
      [
        [0, 1, 5],
        [1, 2, 5],
        [0, 3, 2],
        [3, 1, 2],
        [1, 4, 1],
        [4, 2, 1],
      ],
      0,
      2,
      1,
    ],
    10,
  ],
  [
    [
      5,
      [
        [0, 1, 5],
        [1, 2, 5],
        [0, 3, 2],
        [3, 1, 2],
        [1, 4, 1],
        [4, 2, 1],
      ],
      0,
      2,
      2,
    ],
    7,
  ],
  [
    [
      5,
      [
        [0, 1, 5],
        [1, 2, 5],
        [0, 3, 2],
        [3, 1, 2],
        [1, 4, 1],
        [4, 2, 1],
      ],
      0,
      2,
      3,
    ],
    6,
  ],
];

describe('findCheapestPrice', () => {
  test('it should return the cheapest price for a flight in k or fewer stops', () => {
    testTuples.forEach(([args, expected]) => {
      expect(findCheapestPrice(...args)).toBe(expected);
    });
  });
});
