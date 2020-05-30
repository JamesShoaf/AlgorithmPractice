const {
  floodFill,
} = require('./floodFill');

describe('floodFill', () => {
  const inputs = [
    [
      [
        [1, 1, 1],
        [1, 1, 0],
        [1, 0, 1],
      ],
      1, 1, 2,
    ],
    [
      [
        [0, 0, 0],
        [0, 1, 1],
      ],
      1, 1, 1,
    ],
  ];

  const expected = [
    [
      [2, 2, 2],
      [2, 2, 0],
      [2, 0, 1],
    ],
    [
      [0, 0, 0],
      [0, 1, 1],
    ],
  ];
  test('it should paint the array', () => {
    for (let i = 0; i < inputs.length; i += 1) {
      expect(floodFill(...inputs[i])).toEqual(expected[i]);
    }
  });
});
