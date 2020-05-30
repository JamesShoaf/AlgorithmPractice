// Given a string containing only three types of characters: '(', ')' and '*',
// write a function to check whether this string is valid.
// We define the validity of a string by these rules:

// 1. Any left parenthesis '(' must have a corresponding right parenthesis ')'.
// 2. Any right parenthesis ')' must have a corresponding left parenthesis '('.
// 3. Left parenthesis '(' must go before the corresponding right parenthesis ')'.
// 4. '*' could be treated as a single right parenthesis ')'
//   or a single left parenthesis '(' or an empty string.
// 5. An empty string is also valid.

// scan forwards and backwards
// star forward pointer
// star backward pointer

// simplify the string through a preliminary scan
// if a ) is the first character, return false
// slice out any '()' that occur

// const validParenthesisString = (string) => {
//   // remove self-closing ()s and terminal (* or *)s
//   const innerReducer = (subString) => {
//     const reducedString = subString
//       .replace(/^\*\)/, '') // match *) at start of string
//       .replace(/\(\*$/, '') // match (* at end of string
//       .replace(/\(\)/g, ''); // match ()
//     return (reducedString === subString)
//       ? subString
//       : innerReducer(reducedString);
//   };

//   // remove enclosing ()s
//   const outerReducer = (subString) => {
//     const reducedString = subString
//       .replace(/^(\(|\*)|\)$/, '') // match start ( or * and end )
//       .replace(/^\(|(\*|\))$/, ''); // match start ( and end * or )
//     return (reducedString === subString)
//       ? subString
//       : outerReducer(reducedString);
//   };

//   // recursively checks if there is an uncloseable ) or (, then reduces the string using helpers
//   const recursiveStringReducer = (subString) => {
//     if (subString[subString.length - 1] === '(') {
//       return '! ERROR ( at end';
//     }
//     if (subString[0] === ')') {
//       return '! ERROR ) at start';
//     }
//     let reducedString = innerReducer(subString);
//     reducedString = outerReducer(reducedString);
//     return (reducedString === subString)
//       ? subString
//       : recursiveStringReducer(reducedString);
//   };

//   const reduced = recursiveStringReducer(string);
//   return reduced[0] !== '!';
// };

const validParenthesisString = (string) => {
  const charStack = {
    '(': 0,
    '*': 0,
    '(*': 0,
  };

  for (let i = 0; i < string.length; i += 1) {
    switch (string[i]) {
      case '(':
        charStack['('] += 1;
        break;
      case '*':
        if (charStack['(']) {
          charStack['('] -= 1;
          charStack['(*'] += 1;
        } else {
          charStack['*'] += 1;
        }
        break;
      case ')':
        if (charStack['(']) {
          charStack['('] -= 1;
        } else if (charStack['(*']) {
          charStack['(*'] -= 1;
          charStack['*'] += 1;
        } else if (charStack['*']) {
          charStack['*'] -= 1;
        } else {
          return false;
        }
        break;
      default:
        return false;
    }
  }
  return charStack['('] === 0;
};

validParenthesisString('((()))()(())(*()()())**(())()()()()((*()*))((*()*)');

module.exports = {
  validParenthesisString,
};
