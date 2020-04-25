const mentaculus = require('./mentaculus');

describe('generateSkipAscending', () => {
  test('it should return a string of the correct length', () => {
    for (let i = 1; i < 31; i += 1) {
      const ascending = mentaculus.generateSkipAscending(i);
      expect(typeof ascending).toBe('string');
      expect(ascending.length).toBe(i);
    }
  });

  test('it should return the correct string', () => {
    const testString = '123456789123456789123456789123456789123456789123456789';
    for (let i = 1; i < 31; i += 1) {
      const subString = testString.slice(0, i);
      expect(mentaculus.generateSkipAscending(i)).toBe(subString);
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

  test('it should return the correct string', () => {
    const testString = '123456789012345678901234567890123456789012345678901234567890';
    for (let i = 1; i < 31; i += 1) {
      const subString = testString.slice(0, i);
      expect(mentaculus.generateAscending(i)).toBe(subString);
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
