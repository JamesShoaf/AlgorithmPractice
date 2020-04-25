const mentaculus = require('./mentaculus');

describe('generateSkipAscending', () => {
  test('it should return a string of the correct length', () => {
    for (let i = 1; i < 31; i += 1) {
      const ascending = mentaculus.generateSkipAscending(i);
      expect(typeof ascending).toBe('string');
      expect(ascending.length).toBe(i);
    }
  });

  test('it should return a string of the correct length for non-default bases', () => {
    for (let i = 1; i < 31; i += 1) {
      for (let j = 3; j < 10; j += 1) {
        const ascending = mentaculus.generateSkipAscending(i, j);
        expect(typeof ascending).toBe('string');
        expect(ascending.length).toBe(i);
      }
    }
  });

  test('it should return the correct string', () => {
    const testStrings = {
      3: '12121212121212121212121212121212121212121212121212121212',
      4: '123123123123123123123123123123123123123123123123123123123123',
      5: '1234123412341234123412341234123412341234123412341234123412341234',
      6: '123451234512345123451234512345123451234512345123451234512345',
      7: '123456123456123456123456123456123456123456123456123456123456',
      8: '123456712345671234567123456712345671234567123456712345671234567',
      9: '1234567812345678123456781234567812345678123456781234567812345678',
      10: '123456789123456789123456789123456789123456789123456789',
    };
    for (let i = 1; i < 31; i += 1) {
      for (let j = 3; j < 11; j += 1) {
        const subString = testStrings[j].slice(0, i);
        expect(mentaculus.generateSkipAscending(i, j)).toBe(subString);
      }
    }
  });
});

describe('generateAscending', () => {
  test('it should return a string of the correct length', () => {
    for (let i = 1; i < 31; i += 1) {
      const ascending = mentaculus.generateAscending(i);
      expect(typeof ascending).toBe('string');
      expect(ascending.length).toBe(i);
    }
  });

  test('it should return a string of the correct length for non-default bases', () => {
    for (let i = 1; i < 31; i += 1) {
      for (let j = 3; j < 10; j += 1) {
        const ascending = mentaculus.generateAscending(i, j);
        expect(typeof ascending).toBe('string');
        expect(ascending.length).toBe(i);
      }
    }
  });

  test('it should return the correct string', () => {
    const testStrings = {
      3: '120120120120120120120120120120120120120120120120120120120120',
      4: '123012301230123012301230123012301230123012301230123012301230',
      5: '123401234012340123401234012340123401234012340123401234012340',
      6: '123450123450123450123450123450123450123450123450123450123450',
      7: '123456012345601234560123456012345601234560123456012345601234560',
      8: '1234567012345670123456701234567012345670123456701234567012345670',
      9: '123456780123456780123456780123456780123456780123456780123456780',
      10: '123456789012345678901234567890123456789012345678901234567890',
    };
    for (let i = 1; i < 31; i += 1) {
      for (let j = 3; j < 11; j += 1) {
        const subString = testStrings[j].slice(0, i);
        expect(mentaculus.generateAscending(i, j)).toBe(subString);
      }
    }
  });
});

describe('generateSkipDescending', () => {
  test('it should return a string of the correct length', () => {
    for (let i = 1; i < 31; i += 1) {
      const ascending = mentaculus.generateSkipDescending(i);
      expect(typeof ascending).toBe('string');
      expect(ascending.length).toBe(i);
    }
  });

  test('it should return the correct string', () => {
    const testString = '987654321987654321987654321987654321987654321987654321';
    for (let i = 1; i < 31; i += 1) {
      const subString = testString.slice(0, i);
      expect(mentaculus.generateSkipDescending(i)).toBe(subString);
    }
  });
});

describe('generateDescending', () => {
  test('it should return a string of the correct length', () => {
    for (let i = 1; i < 31; i += 1) {
      const ascending = mentaculus.generateDescending(i);
      expect(typeof ascending).toBe('string');
      expect(ascending.length).toBe(i);
    }
  });

  test('it should return the correct string', () => {
    const testString = '987654321098765432109876543210987654321098765432109876543210';
    for (let i = 1; i < 31; i += 1) {
      const subString = testString.slice(0, i);
      expect(mentaculus.generateDescending(i)).toBe(subString);
    }
  });
});

describe('skipRemainder', () => {
  test('it should return a string', () => {
    for (let i = 1; i < 31; i += 1) {
      expect(typeof mentaculus.skipRemainder(i)).toBe('string');
    }
  });

  test('it should return the remainder that must be added to the multiple of the ascending string', () => {
    for (let i = 1; i < 10; i += 1) {
      expect(mentaculus.skipRemainder(i)).toBe(`${i}`);
    }
  });
});

describe('remainder', () => {
  test('it should return a string', () => {
    for (let i = 1; i < 31; i += 1) {
      expect(typeof mentaculus.remainder(i)).toBe('string');
    }
  });

  test('it should return the remainder that must be added to the multiple of the ascending string', () => {
    for (let i = 1; i < 10; i += 1) {
      expect(mentaculus.remainder(i)).toBe(`${i}`);
    }
  });
});
