const { twoCitySchedCost } = require('./twoCitySchedCost');

const testTuples = [
  [
    [
      [10, 20],
      [30, 200],
      [400, 50],
      [30, 20],
    ],
    110,
  ],
  [
    [
      [-5, 100],
      [-5, 200],
      [-5, 300],
      [-5, 400],
    ],
    290,
  ],
  [
    [
      [100, -5],
      [100, -10],
      [100, -15],
      [100, -20],
    ],
    165,
  ],
];

describe('twoCitySchedCosts', () => {
  test('it should return the correct number', () => {
    testTuples.forEach(([costs, expected]) => {
      expect(twoCitySchedCost(costs)).toBe(expected);
    });
  });

  test('it should throw an error if provided an odd-length array', () => {
    expect(() => twoCitySchedCost([[1, 2]])).toThrow();
  });

  test('it should throw an error if provided a missing or invalid cost', () => {
    expect(() => twoCitySchedCost([[1, 2], [3]])).toThrow();
    expect(() => twoCitySchedCost([[1, 'hello'], [3, 4]])).toThrow();
  });
});
