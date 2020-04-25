const {
  RadInt,
} = require('./RadInt');

const rad0 = new RadInt();
const rad3 = new RadInt('3', 10);
const rad5 = new RadInt('5', 7);
const rad9 = new RadInt('12', 7);
const testArray = [rad0, rad3, rad5];

describe('Constructor', () => {
  test('it should generate RadInt objects', () => {
    testArray.forEach((radInt) => expect(radInt instanceof RadInt).toBe(true));
    expect(rad0.value).toBe('0');
    expect(rad3.value).toBe('3');
    expect(rad5.value).toBe('5');
    expect(rad9.value).toBe('12');
    expect(rad0.radix).toBe(10);
    expect(rad3.radix).toBe(10);
    expect(rad5.radix).toBe(7);
    expect(rad9.radix).toBe(7);
  });
});

describe('minus', () => {
  test('it should return null if the two numbers have a base mismatch', () => {
    expect(rad9.minus(rad0)).toBe(null);
    expect(rad5.minus(rad3)).toBe(null);
  });

  test('it should return a RadInt', () => {
    expect(rad3.minus(rad0) instanceof RadInt).toBe(true);
    expect(rad9.minus(rad5) instanceof RadInt).toBe(true);
  });

  test('it should return a RadInt with the same base', () => {
    expect(rad3.minus(rad0).radix).toBe(10);
    expect(rad9.minus(rad5).radix).toBe(7);
  });

  test('it should return a RadInt with the correct value', () => {
    expect(rad3.minus(rad0).value).toBe('3');
    expect(rad9.minus(rad5).value).toBe('4');
  });
});
