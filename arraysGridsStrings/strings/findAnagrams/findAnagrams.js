/*
Given a string s and a non-empty string p, find all the start indices of p's anagrams in s.

Strings consists of lowercase English letters only and the length of both strings s and p
will not be larger than 20,100.

The order of output does not matter.
*/

const findAnagrams = (s, p) => {
  const { length: sLength } = s;
  const { length: pLength } = p;
  if (pLength > sLength) return [];

  const abecedary = 'abcdefghijklmnopqrstuvwxyz';
  const charCounts = {};

  // add all 26 characters to charCounts
  for (let i = 0; i < 26; i += 1) {
    charCounts[abecedary[i]] = 0;
  }
  // add characters from p to charCounts
  for (let i = 0; i < pLength; i += 1) {
    charCounts[p[i]] += 1;
  }

  // iterate through string for matches
  const anagramIndices = [];
  for (let i = 0; i < sLength; i += 1) {
    charCounts[s[i]] -= 1;
    if (i >= pLength - 1) {
      const tail = i + 1 - pLength;
      let isAnagram = true;
      for (let j = 0; j < 26; j += 1) {
        if (charCounts[abecedary[j]] !== 0) {
          isAnagram = false;
          break;
        }
      }
      if (isAnagram) anagramIndices.push(tail);
      charCounts[s[tail]] += 1;
    }
  }
  return anagramIndices;
};

module.exports = { findAnagrams };
