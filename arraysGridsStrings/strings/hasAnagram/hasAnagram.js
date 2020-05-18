/*
Given two strings s1 and s2, write a function to return true if s2 contains the permutation of s1.
In other words, one of the first string's permutations is the substring of the second string.
*/

const hasAnagram = (s1, s2) => {
  const { length: s1Length } = s1;
  if (s1.length === 0) return true;
  const { length: s2Length } = s2;
  if (s1Length > s2Length) return false;
  const counts = {};
  for (let i = 0; i < s1Length; i += 1) {
    counts[s1[i]] = counts[s1[i]] + 1 || 1;
  }
  const charArray = Object.keys(counts);
  const charArrayLength = charArray.length;
  for (let i = 0; i < s2Length; i += 1) {
    const tail = i - s1Length;
    if (counts[s2[tail]] !== undefined) counts[s2[tail]] += 1;
    if (counts[s2[i]] !== undefined) {
      counts[s2[i]] -= 1;
      if (counts[s2[i]] === 0) {
        let foundFlag = true;
        for (let j = 0; j < charArrayLength; j += 1) {
          if (counts[charArray[j]] !== 0) {
            foundFlag = false;
            break;
          }
        }
        if (foundFlag === true) return true;
      }
    }
  }
  return false;
};

module.exports = { hasAnagram };
