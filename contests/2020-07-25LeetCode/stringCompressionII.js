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

const getLengthOfOptimalCompression = (s, k) => {
  const { length } = s;
  if (k >= length) return 0;
  // memoize the best compressed string length for strings from 0 to index j
  // with 0 to k removed characters
  const memo = [...Array(length)].map(() => [...Array(k + 1)]);

  const charLength = (count) => ((count <= 2) ? count : Math.ceil(Math.log10(count + 1)) + 1);

  const compressedLength = (index, remainingDeletes) => {
    // if the end of the string is reached or all remaining characters can be deleted, return 0
    if (index === length || length - index <= remainingDeletes) return 0;
    // return memoized answers
    if (memo[index][remainingDeletes]) return memo[index][remainingDeletes];
    // count characters forward from the index, tracking how often the most frequent one appears
    const frequencyCounter = {};
    let most = 0;
    // initialize output as uncompressed length
    let output = length;
    for (let i = index; i < length; i += 1) {
      const char = s[i];
      if (!frequencyCounter[char]) frequencyCounter[char] = 0;
      frequencyCounter[char] += 1;
      most = Math.max(most, frequencyCounter[char]);
      // count how many characters other than the most frequent one appear
      const charsToDelete = i - index + 1 - most;
      if (remainingDeletes >= charsToDelete) {
        // and compare the result of deleting those characters with the current best length
        output = Math.min(
          output,
          charLength(most) + compressedLength(i + 1, remainingDeletes - charsToDelete),
        );
      // if there are more characters to delete in the remaining segment than can be deleted
      // stop counting characters
      } else { break; }
    }
    // memoize the result and return it
    memo[index][remainingDeletes] = output;
    return output;
  };

  return compressedLength(0, k);
};

getLengthOfOptimalCompression(
  'abcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxy',
  99,
);
