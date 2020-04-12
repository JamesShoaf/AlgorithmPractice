const ttkm = require('./touchToneKnightsMove');

describe('TouchTone Knights\' Move', () => {
  test('it should return null when passed a non-integer', () => {
    expect(ttkm('#', 3)).toBe(null);
    expect(ttkm('*', 3)).toBe(null);
    expect(ttkm(null, 3)).toBe(null);
    expect(ttkm(undefined, 3)).toBe(null);
    expect(ttkm('', 3)).toBe(null);
    expect(ttkm(1.1, 3)).toBe(null);
  });

  test('it should return 1 when passed an integer and 0 steps', () => {
    for (let i = 0; i < 10; i += 1) {
      expect(ttkm(i, 0)).toBe(1);
    }
  });

  test('it should return the number of possible moves from a number when passed that number and 1 steps', () => {
    expect(ttkm(0, 1)).toBe(2);
    expect(ttkm(1, 1)).toBe(2);
    expect(ttkm(2, 1)).toBe(2);
    expect(ttkm(3, 1)).toBe(2);
    expect(ttkm(4, 1)).toBe(3);
    expect(ttkm(5, 1)).toBe(0);
    expect(ttkm(6, 1)).toBe(3);
    expect(ttkm(7, 1)).toBe(1);
    expect(ttkm(8, 1)).toBe(2);
    expect(ttkm(9, 1)).toBe(1);
  });

  test('it should return the number of possible moves from a number when passed that number and 2 steps', () => {
    expect(ttkm(0, 2)).toBe(6);
    expect(ttkm(1, 2)).toBe(5);
    expect(ttkm(2, 2)).toBe(2);
    expect(ttkm(3, 2)).toBe(5);
    expect(ttkm(4, 2)).toBe(5);
    expect(ttkm(5, 2)).toBe(0);
    expect(ttkm(6, 2)).toBe(5);
    expect(ttkm(7, 2)).toBe(3);
    expect(ttkm(8, 2)).toBe(4);
    expect(ttkm(9, 2)).toBe(3);
  });
});
