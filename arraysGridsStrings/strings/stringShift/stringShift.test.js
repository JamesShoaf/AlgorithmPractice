const {
  stringShift,
} = require('./stringShift');

describe('stringShift', () => {
  const testStrings = {
    0: 't',
    1: 'test',
    2: 'atcgatcgatcg',
  };

  const testShifts = {
    0: [[0, 0]], // left 0
    1: [[1, 0]], // right 0
    2: [[0, 1], [1, 1]], // left right 0
    3: [[0, 1]], // left 1
    4: [[1, 1]], // right 1
    5: [[0, 2], [1, 1]], // left 2, right 1 => left 1
    6: [[0, 1], [1, 2]], // left 1, right 2 => right 1
  };

  test('it should return a string', () => {
    for (let i = 0; i < Object.keys(testStrings).length; i += 1) {
      for (let j = 0; j < Object.keys(testShifts).length; j += 1) {
        expect(typeof stringShift(testStrings[i], testShifts[j])).toBe('string');
      }
    }
  });

  test('it should shift the string the correct number of places', () => {
    for (let i = 0; i < Object.keys(testShifts).length; i += 1) {
      expect(stringShift(testStrings[0], testShifts[i])).toBe('t');
    }

    expect(stringShift(testStrings[1], testShifts[0])).toBe('test');
    expect(stringShift(testStrings[1], testShifts[1])).toBe('test');
    expect(stringShift(testStrings[1], testShifts[2])).toBe('test');
    expect(stringShift(testStrings[1], testShifts[3])).toBe('estt');
    expect(stringShift(testStrings[1], testShifts[4])).toBe('ttes');
    expect(stringShift(testStrings[1], testShifts[5])).toBe('estt');
    expect(stringShift(testStrings[1], testShifts[6])).toBe('ttes');

    expect(stringShift(testStrings[2], testShifts[0])).toBe('atcgatcgatcg');
    expect(stringShift(testStrings[2], testShifts[1])).toBe('atcgatcgatcg');
    expect(stringShift(testStrings[2], testShifts[2])).toBe('atcgatcgatcg');
    expect(stringShift(testStrings[2], testShifts[3])).toBe('tcgatcgatcga');
    expect(stringShift(testStrings[2], testShifts[4])).toBe('gatcgatcgatc');
    expect(stringShift(testStrings[2], testShifts[5])).toBe('tcgatcgatcga');
    expect(stringShift(testStrings[2], testShifts[6])).toBe('gatcgatcgatc');
  });
});
