// string matching in an array

/* Given an array of string words. Return all strings in words which is
 substring of another word in any order.

String words[i] is substring of words[j], if can be obtained removing some
characters to left and/or right side of words[j]. */

/*
For each word in the array
  check that word against all other words
  in case of an exact match, continue
  in case of a substring match, push the word and break
*/

const stringMatcher = (words) => {
  const output = [];
  for (let i = 0; i < words.length; i += 1) {
    const substring = words[i];
    for (let j = 0; j < words.length; j += 1) {
      const currentWord = words[j];
      if (currentWord.indexOf(substring) !== -1 && substring !== currentWord) {
        output.push(substring);
        break;
      }
    }
  }
  return output;
};

module.exports = stringMatcher;
