const {
  RadInt,
} = require('./RadInt');

const testValues = [];
for (let int = 0; int < Number.MAX_SAFE_INTEGER; int *= 10) {
  testValues.push(int);
  int += 1;
  testValues.push(int);
}
// [0, 1, 10, 11, 110, 111, 1110, 1111, 11110, 11111, 111110, 111111, 1111110, 1111111, ...]
// powersOf10[15] < Math.sqrt(Number.MAX_SAFE_INTEGER) < powersOf10[16]

const intsArray = [];
for (let base = 3; base <= 10; base += 1) {
  const baseArray = testValues.map((int) => new RadInt(int, base));
  intsArray.push(baseArray);
}
const rad0 = new RadInt();
const rad3 = new RadInt('3', 10);
const rad5 = new RadInt('5', 7);
const rad9 = new RadInt('12', 7);
const stringsArray = [rad0, rad3, rad5, rad9];

describe('Constructor', () => {
  test('it should generate RadInt objects', () => {
    intsArray.forEach((baseRow) => {
      baseRow.forEach((radInt) => {
        expect(radInt instanceof RadInt).toBe(true);
      });
    });
    stringsArray.forEach((radInt) => expect(radInt instanceof RadInt).toBe(true));
  });

  test('it should generate RadInts with the correct base', () => {
    intsArray.forEach((baseRow, rowIndex) => {
      const base = rowIndex + 3;
      baseRow.forEach((radInt) => {
        expect(radInt.radix).toBe(base);
      });
    });

    expect(rad0.radix).toBe(10);
    expect(rad3.radix).toBe(10);
    expect(rad5.radix).toBe(7);
    expect(rad9.radix).toBe(7);
  });

  test('it should generate RadInts with the correct value', () => {
    intsArray.forEach((baseRow, rowIndex) => {
      const base = rowIndex + 3;
      baseRow.forEach((radInt, colIndex) => {
        expect(radInt.value).toBe(Number(testValues[colIndex]).toString(base));
      });
    });

    expect(rad0.value).toBe('0');
    expect(rad3.value).toBe('3');
    expect(rad5.value).toBe('5');
    expect(rad9.value).toBe('12');
  });

  test('it should return null if given invalid input', () => {
    const invalidValues = ['', 'hello', 3.14, -2, NaN, true, false, null];
    const invalidBases = ['', 'hello', 3.14, -2, NaN, true, false, null];
    invalidValues.forEach((value) => {
      expect(new RadInt(value, 10)).toBe(null);
    });
    invalidBases.forEach((base) => {
      expect(new RadInt(10, base)).toBe(null);
    });
  });
});

describe('plus', () => {
  test('it should return null if the two numbers have a base mismatch', () => {
    const base3 = intsArray[0];
    for (let row = 1; row < intsArray.length; row += 1) {
      for (let col = 0; col < base3.length; col += 1) {
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
        expect(row[col - 1].plus(row[col]) instanceof RadInt).toBe(true);
        expect(row[col].plus(row[col - 1]) instanceof RadInt).toBe(true);
      }
    });

    expect(rad3.plus(rad0) instanceof RadInt).toBe(true);
    expect(rad9.plus(rad5) instanceof RadInt).toBe(true);
  });

  test('it should return a RadInt with the same base', () => {
    intsArray.forEach((row, rowIndex) => {
      const base = rowIndex + 3;
      for (let col = 1; col < row.length; col += 1) {
        expect(row[col - 1].plus(row[col]).radix).toBe(base);
        expect(row[col].plus(row[col - 1]).radix).toBe(base);
      }
    });

    expect(rad3.plus(rad0).radix).toBe(10);
    expect(rad9.plus(rad5).radix).toBe(7);
  });

  test('it should return a RadInt with the correct value', () => {
    intsArray.forEach((row, rowIndex) => {
      const base = rowIndex + 3;
      const { length } = row;
      for (let col = 1; col < length; col += 1) {
        expect(row[col - 1].plus(row[col]).value)
          .toBe(Number(testValues[col] + testValues[col - 1]).toString(base));
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
      for (let col = 0; col < base3.length; col += 1) {
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
        expect(row[col].minus(row[col - 1]) instanceof RadInt).toBe(true);
      }
    });
    expect(rad3.minus(rad0) instanceof RadInt).toBe(true);
    expect(rad9.minus(rad5) instanceof RadInt).toBe(true);
  });

  test('it should return a RadInt with the same base', () => {
    intsArray.forEach((row, rowIndex) => {
      const base = rowIndex + 3;
      for (let col = 1; col < row.length; col += 1) {
        expect(row[col].minus(row[col - 1]).radix).toBe(base);
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
        expect(row[col].minus(row[col - 1]).value)
          .toBe(Number(testValues[col] - testValues[col - 1]).toString(base));
      }
    });
    expect(rad3.minus(rad0).value).toBe('3');
    expect(rad9.minus(rad5).value).toBe('4');
  });
});

describe('times', () => {
  test('it should return null if the two numbers have a base mismatch', () => {
    const base3 = intsArray[0];
    for (let row = 1; row < intsArray.length; row += 1) {
      for (let col = 0; col < 16; col += 1) {
        expect(base3[col].times(intsArray[row][col])).toBe(null);
        expect(intsArray[row][col].times(base3[col])).toBe(null);
      }
    }
    expect(rad9.times(rad0)).toBe(null);
    expect(rad5.times(rad3)).toBe(null);
  });

  test('it should return a RadInt', () => {
    intsArray.forEach((row) => {
      for (let col = 1; col < 16; col += 1) {
        expect(row[col - 1].times(row[col]) instanceof RadInt).toBe(true);
      }
    });
    expect(rad3.times(rad0) instanceof RadInt).toBe(true);
    expect(rad9.times(rad5) instanceof RadInt).toBe(true);
  });

  test('it should return a RadInt with the same base', () => {
    intsArray.forEach((row, rowIndex) => {
      const base = rowIndex + 3;
      for (let col = 1; col < 16; col += 1) {
        expect(row[col].times(row[col - 1]).radix).toBe(base);
      }
    });
    expect(rad3.times(rad0).radix).toBe(10);
    expect(rad9.times(rad5).radix).toBe(7);
  });

  test('it should return a RadInt with the correct value', () => {
    intsArray.forEach((row, rowIndex) => {
      const base = rowIndex + 3;
      for (let col = 1; col < 16; col += 1) {
        expect(row[col].times(row[col - 1]).value)
          .toBe(Number(testValues[col] * testValues[col - 1]).toString(base));
      }
    });
    expect(rad3.minus(rad0).value).toBe('3');
    expect(rad9.minus(rad5).value).toBe('4');
  });
});
