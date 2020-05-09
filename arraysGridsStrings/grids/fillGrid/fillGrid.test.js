const {
  fillGrid,
} = require('./fillGrid');

const testTuples = [
  [
    [1, 1, 1],
    [[1]],
  ],
  [
    [3, 5],
    [
      [0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0],
    ],
  ],
  [
    [{ rows: 5, cols: 3, fill: 5 }],
    [
      [5, 5, 5],
      [5, 5, 5],
      [5, 5, 5],
      [5, 5, 5],
      [5, 5, 5],
    ],
  ],
  [
    [{ rows: 4, cols: 4 }],
    [
      [0, 0, 0, 0],
      [0, 0, 0, 0],
      [0, 0, 0, 0],
      [0, 0, 0, 0],
    ],
  ],
];

const invalidInputs = [
  -1,
  0,
  1.5,
  NaN,
  null,
  'foo',
  true,
  false,
  [],
  {},
  Symbol('foo'),
];

describe('fillGrid', () => {
  test('it should throw an error if given invalid input', () => {
    invalidInputs.forEach((badInput) => {
      expect(() => fillGrid([1, badInput])).toThrow();
      expect(() => fillGrid([badInput, 1])).toThrow();
    });
  });

  const received = testTuples.map((tuple) => fillGrid(...tuple[0]));
  test('it should return an array', () => {
    received.forEach((result) => {
      expect(Array.isArray(result)).toBe(true);
    });
  });

  test('it should return an array with the correct number of rows', () => {
    testTuples.forEach((tuple, index) => {
      const expected = tuple[1];
      expect(received[index].length).toBe(expected.length);
    });
  });

  test('it should return an array with the correct number of columns', () => {
    testTuples.forEach((tuple, index) => {
      const expected = tuple[1];
      expect(received[index][0].length).toBe(expected[0].length);
    });
  });

  test('it should fill all cells with either 0 or the specified value', () => {
    testTuples.forEach((tuple, index) => {
      const expected = tuple[1];
      expect(received[index]).toEqual(expected);
    });
  });
});
