const {
  cakeThief,
  zeroOneCakesack,
} = require('./cakeThief');

const cakesacks = [...new Array(11).keys()];

const cakeTypes = [
  { weight: 2, value: 15 },
  { weight: 3, value: 90 },
  { weight: 7, value: 160 },
];

describe('cakeThief', () => {
  test('it should return positive infinity if given access to hovercake', () => {
    expect(cakeThief(0, [{ weight: 0, value: 1 }])).toBe(Number.POSITIVE_INFINITY);
  });

  test('it should return the maximum value for unlimited supplies of cake', () => {
    const expected = [0, 0, 15, 90, 90, 105, 180, 180, 195, 270, 270];
    cakesacks.forEach((sack, index) => {
      expect(cakeThief(sack, cakeTypes)).toBe(expected[index]);
    });
  });
});

describe('zeroOneCakesack', () => {
  test('it should return the maximum value for unlimited supplies of cake', () => {
    const expected = [0, 0, 15, 90, 90, 105, 105, 160, 160, 175, 250];
    cakesacks.forEach((sack, index) => {
      expect(zeroOneCakesack(sack, cakeTypes)).toBe(expected[index]);
    });
  });
});
