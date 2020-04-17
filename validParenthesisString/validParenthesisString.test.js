const { validParenthesisString } = require('./validParenthesisString');

describe('validParenthesisString', () => {
  const testValues = [
    ['', true],
    ['(', false],
    [')', false],
    ['(*', true],
    ['*)', true],
    ['()', true],
    [')(', false],
    ['(*)', true],
    ['**)', true],
    ['()()', true],
    ['(())((())()()(*)(*()(())())())()()((()())((()))(*', false],
    ['(()*)(()((())()))(*)((((())*())))()(((()((()(*()))', false],
    ['((()))()(())(*()()())**(())()()()()((*()*))((*()*)', true],
  ];

  test('it returns a Boolean', () => {
    testValues.forEach((val) => {
      expect(typeof validParenthesisString(val[0])).toBe('boolean');
    });
  });

  test('it returns true when the string contains a matching set of open and close parentheses', () => {
    testValues.forEach((val) => {
      expect(validParenthesisString(val[0])).toBe(val[1]);
    });
  });
});
