const {
  RadInt,
} = require('./RadInt');

const intsArray = [];
for (let base = 3; base <= 10; base += 1) {
  intsArray.push([]);
  const baseArray = intsArray[base - 3];
  for (let int = 0; int < 1000; int += 1) {
    baseArray.push(new RadInt(int, base));
  }
}
const rad0 = new RadInt();
const rad3 = new RadInt('3', 10);
const rad5 = new RadInt('5', 7);
const rad9 = new RadInt('12', 7);
const stringsArray = [rad0, rad3, rad5, rad9];

describe('Constructor', () => {
  test('it should generate RadInt objects', () => {
    intsArray.forEach((baseRow, rowIndex) => {
      const base = rowIndex + 3;
      baseRow.forEach((radInt, colIndex) => {
        expect(radInt instanceof RadInt).toBe(true);
        expect(radInt.value).toBe(Number(colIndex).toString(base));
        expect(radInt.radix).toBe(base);
      });
    });
    stringsArray.forEach((radInt) => expect(radInt instanceof RadInt).toBe(true));
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

describe('plus', () => {
  test('it should return null if the two numbers have a base mismatch', () => {
    const base3 = intsArray[0];
    for (let row = 1; row < intsArray.length; row += 1) {
      for (let col = 0; col < intsArray[0].length; col += 1) {
        expect(base3[col].plus(intsArray[row][col])).toBe(null);
        expect(intsArray[row][col].plus(base3[col])).toBe(null);
      }
    }

    expect(rad9.plus(rad0)).toBe(null);
    expect(rad5.plus(rad3)).toBe(null);
  });

  test('it should return a RadInt', () => {
    intsArray.forEach((row) => {
      for (let col = 1; col < row.length; col += 1) {
        expect(row[0].plus(row[col]) instanceof RadInt).toBe(true);
        expect(row[col].plus(row[0]) instanceof RadInt).toBe(true);
      }
    });
    expect(rad3.plus(rad0) instanceof RadInt).toBe(true);
    expect(rad9.plus(rad5) instanceof RadInt).toBe(true);
  });

  test('it should return a RadInt with the same base', () => {
    intsArray.forEach((row, rowIndex) => {
      const base = rowIndex + 3;
      for (let col = 1; col < row.length; col += 1) {
        expect(row[0].plus(row[col]).radix).toBe(base);
        expect(row[col].plus(row[0]).radix).toBe(base);
      }
    });

    expect(rad3.plus(rad0).radix).toBe(10);
    expect(rad9.plus(rad5).radix).toBe(7);
  });

  test('it should return a RadInt with the correct value', () => {
    intsArray.forEach((row, rowIndex) => {
      const base = rowIndex + 3;
      for (let col = 1; col < row.length; col += 1) {
        expect(row[0].plus(row[col]).value).toBe(Number(col).toString(base));
        expect(row[col].plus(row[0]).value).toBe(Number(col).toString(base));
      }
    });

    expect(rad3.plus(rad0).value).toBe('3');
    expect(rad9.plus(rad5).value).toBe('20');
  });
});

describe('minus', () => {
  test('it should return null if the two numbers have a base mismatch', () => {
    const base3 = intsArray[0];
    for (let row = 1; row < intsArray.length; row += 1) {
      for (let col = 0; col < intsArray[0].length; col += 1) {
        expect(base3[col].minus(intsArray[row][col])).toBe(null);
        expect(intsArray[row][col].minus(base3[col])).toBe(null);
      }
    }
    expect(rad9.minus(rad0)).toBe(null);
    expect(rad5.minus(rad3)).toBe(null);
  });

  test('it should return a RadInt', () => {
    intsArray.forEach((row) => {
      for (let col = 1; col < row.length; col += 1) {
        expect(row[col].minus(row[0]) instanceof RadInt).toBe(true);
      }
    });
    expect(rad3.minus(rad0) instanceof RadInt).toBe(true);
    expect(rad9.minus(rad5) instanceof RadInt).toBe(true);
  });

  test('it should return a RadInt with the same base', () => {
    intsArray.forEach((row, rowIndex) => {
      const base = rowIndex + 3;
      for (let col = 1; col < row.length; col += 1) {
        expect(row[col].minus(row[0]).radix).toBe(base);
      }
    });
    expect(rad3.minus(rad0).radix).toBe(10);
    expect(rad9.minus(rad5).radix).toBe(7);
  });

  test('it should return a RadInt with the correct value', () => {
    intsArray.forEach((row, rowIndex) => {
      const base = rowIndex + 3;
      const { length } = row;
      for (let col = 1; col < length; col += 1) {
        expect(row[col].minus(row[0]).value).toBe(Number(col).toString(base));
      }
      const biggest = row[length - 1];
      for (let col = length - 2; col > 0; col -= 1) {
        expect(biggest.minus(row[col]).value).toBe(Number(length - col - 1).toString(base));
      }
    });
    expect(rad3.minus(rad0).value).toBe('3');
    expect(rad9.minus(rad5).value).toBe('4');
  });
});
