const {
  bstFromPreorder,
} = require('./bstFromPreorderArray');

describe('bstFromPreorder', () => {
  const testArrays = [
    [
      [8, 5, 1, 7, 10, 12],
      [8, 5, 10, 1, 7, null, 12],
    ],
    [
      [0],
      [0],
    ],
    [
      [2, 1, 3],
      [2, 1, 3],
    ],
    [
      [1, 0],
      [1, 0, null],
    ],
    [
      [3, 2, 1],
      [3, 2, null, 1, null, null, null],
    ],
    [
      [1, 2, 3, 4],
      [1, null, 2, null, null, null, 3, null, null, null, null, null, null, null, 4],
    ],
  ];

  test('it should return an array', () => {
    testArrays.forEach((testTuple) => {
      expect(Array.isArray(bstFromPreorder(testTuple[0]))).toBe(true);
    });
  });

  test('it should return an array which contains all elements of the input array', () => {
    testArrays.forEach((testTuple) => {
      expect(bstFromPreorder(testTuple[0])).toEqual(expect.arrayContaining(testTuple[0]));
    });
  });


  test('it should return a binary search tree from an array', () => {
    testArrays.forEach((testTuple) => {
      expect(bstFromPreorder(testTuple[0])).toEqual(testTuple[1]);
    });
  });
});
