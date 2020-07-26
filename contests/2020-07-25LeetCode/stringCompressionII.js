/* 
Run-length encoding is a string compression method that works by replacing consecutive identical
characters (repeated 2 or more times) with the concatenation of the character and the number
marking the count of the characters (length of the run). For example, to compress the string
"aabccc" we replace "aa" by "a2" and replace "ccc" by "c3". Thus the compressed string becomes
"a2bc3".

Notice that in this problem, we are not adding '1' after single characters.

Given a string s and an integer k. You need to delete at most k characters from s such that the
run-length encoded version of s has minimum length.

Find the minimum length of the run-length encoded version of s after deleting at most k characters.
*/

const compressString = (s) => {
  const { length } = s;
  let lastCharacter;
  let repeatCount = 0;
  let compressed = '';
  for (let i = 0; i < length; i += 1) {
    if (s[i] !== lastCharacter) {
      if (repeatCount > 1) { compressed += repeatCount.toString(10); }
      repeatCount = 1;
      lastCharacter = s[i];
      compressed += s[i];
    } else {
      repeatCount += 1;
    }
  }
  if (repeatCount > 1) { compressed += repeatCount.toString(10); }
  return compressed;
};

const getLengthOfOptimalCompression = (s, k) => {
  if (k === 0) return compressString(s).length;
  const { length } = s;
  let lastCharacter;
  let shortest = length;
  for (let i = 0; i < length; i += 1) {
    if (lastCharacter !== s[i]) {
      lastCharacter = s[i];
      const truncated = s.slice(0, i) + s.slice(i + 1, length);
      shortest = Math.min(shortest, getLengthOfOptimalCompression(truncated, k - 1));
    }
  }
  return shortest;
};

getLengthOfOptimalCompression('abbbbbbbbbba', 2);