const lastStoneWeight = require('./lastStoneWeight');

describe('lastStoneWeight', () => {
  test('it should ignore invalid stone weights', () => {
    expect(lastStoneWeight([-1])).toBe(0);
    expect(lastStoneWeight([-6, -2])).toBe(0);
    expect(lastStoneWeight([-4, 0, 1])).toBe(1);
  });

  test('it should return the original value when only one stone is passed in', () => {
    expect(lastStoneWeight([1])).toBe(1);
    expect(lastStoneWeight([3])).toBe(3);
    expect(lastStoneWeight([6])).toBe(6);
  });

  test('it should return the difference between two stones when only two are passed in', () => {
    expect(lastStoneWeight([1, 2])).toBe(1);
    expect(lastStoneWeight([3, 6])).toBe(3);
    expect(lastStoneWeight([6, 5])).toBe(1);
  });

  test('it should return the appropriate remainder when three or more stones are passed in', () => {
    expect(lastStoneWeight([1, 2, 2])).toBe(1);
    expect(lastStoneWeight([1, 6, 9])).toBe(2);
    expect(lastStoneWeight([2, 4, 6, 8])).toBe(0);
  });
});
