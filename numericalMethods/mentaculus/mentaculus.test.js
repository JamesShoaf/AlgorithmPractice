const {
  mentaculus,
} = require('./mentaculus');

describe('generateSkipAscending', () => {
  test('it should return a string of the correct length', () => {
    for (let i = 1; i < 31; i += 1) {
      for (let j = 3; j < 11; j += 1) {
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
      for (let j = 3; j < 11; j += 1) {
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
      for (let j = 3; j < 11; j += 1) {
        const descending = mentaculus.generateSkipDescending(i, j);
        expect(typeof descending).toBe('string');
        expect(descending.length).toBe(i);
      }
    }
  });

  test('it should return the correct string', () => {
    const testStrings = {
      3: '21212121212121212121212121212121212121212121212121212121',
      4: '321321321321321321321321321321321321321321321321321321321',
      5: '43214321432143214321432143214321432143214321432143214321',
      6: '5432154321543215432154321543215432154321543215432154321',
      7: '654321654321654321654321654321654321654321654321654321654321',
      8: '76543217654321765432176543217654321765432176543217654321',
      9: '87654321876543218765432187654321876543218765432187654321',
      10: '987654321987654321987654321987654321987654321987654321',
    };
    for (let i = 1; i < 31; i += 1) {
      for (let j = 3; j < 11; j += 1) {
        const subString = testStrings[j].slice(0, i);
        expect(mentaculus.generateSkipDescending(i, j)).toBe(subString);
      }
    }
  });
});

describe('generateDescending', () => {
  test('it should return a string of the correct length', () => {
    for (let i = 1; i < 31; i += 1) {
      for (let j = 3; j < 11; j += 1) {
        const descending = mentaculus.generateDescending(i, j);
        expect(typeof descending).toBe('string');
        expect(descending.length).toBe(i);
      }
    }
  });

  test('it should return the correct string', () => {
    const testStrings = {
      3: '210210210210210210210210210210210210210210210210210210210210',
      4: '3210321032103210321032103210321032103210321032103210321032103210',
      5: '43210432104321043210432104321043210432104321043210432104321043210',
      6: '543210543210543210543210543210543210543210543210543210543210543210',
      7: '654321065432106543210654321065432106543210654321065432106543210',
      8: '7654321076543210765432107654321076543210765432107654321076543210',
      9: '876543210876543210876543210876543210876543210876543210876543210',
      10: '987654321098765432109876543210987654321098765432109876543210',
    };
    for (let i = 1; i < 31; i += 1) {
      for (let j = 3; j < 11; j += 1) {
        const subString = testStrings[j].slice(0, i);
        expect(mentaculus.generateDescending(i, j)).toBe(subString);
      }
    }
  });
});

describe('skipRemainder', () => {
  test('it should return a string', () => {
    for (let i = 1; i < 31; i += 1) {
      for (let j = 3; j < 11; j += 1) {
        expect(typeof mentaculus.skipRemainder(i, j)).toBe('string');
      }
    }
  });

  test('it should return the remainder that must be added to the multiple of the ascending string', () => {
    for (let i = 3; i < 11; i += 1) {
      for (let j = 1; j < i; j += 1) {
        expect(mentaculus.skipRemainder(j, i)).toBe(`${j}`);
      }
    }
  });
});

describe('remainder', () => {
  test('it should return a string', () => {
    for (let i = 1; i < 31; i += 1) {
      for (let j = 3; j < 11; j += 1) {
        expect(typeof mentaculus.remainder(i, j)).toBe('string');
      }
    }
  });

  test('it should return the remainder that must be added to the multiple of the ascending string', () => {
    for (let i = 3; i < 11; i += 1) {
      for (let j = 1; j < i; j += 1) {
        expect(mentaculus.remainder(j, i)).toBe(`${j}`);
      }
    }
  });
});
