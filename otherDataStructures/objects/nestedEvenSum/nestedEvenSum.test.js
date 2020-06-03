const { nestedEvenSum } = require('./nestedEvenSum');

const testTuples = [
  [
    {
      outer: 2,
      obj: {
        inner: 2,
        otherObj: {
          superInner: 2,
          notANumber: true,
          alsoNotANumber: 'yup',
        },
      },
    },
    6,
  ],
  [
    {
      a: 2,
      b: { b: 2, bb: { b: 3, bb: { b: 2 } } },
      c: { c: { c: 2 }, cc: 'ball', ccc: 5 },
      d: 1,
      e: { e: { e: 2 }, ee: 'car' },
    },
    10,
  ],
];

describe('nestedEvenSum', () => {
  test('it should return the sum of all properties that are even numbers', () => {
    testTuples.forEach(([obj, expected]) => {
      expect(nestedEvenSum(obj)).toBe(expected);
    });
  });
});
