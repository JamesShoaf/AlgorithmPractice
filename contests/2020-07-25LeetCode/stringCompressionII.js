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
  // create a set of values which incur a length cost for repeated characters
  const lengthCost = new Set([1]);
  // eg, for a string of 1000 As, the length increases would occur at A, A2, A10, A100, and A1000
  for (let i = 10; i < length; i *= 10) lengthCost.add(i - 1);
  // start index, last letter added to string, repeated letter count, deletions left
  const compressedLength = (start, last, count, left) => {
    // set an infinite cost for deleting more than k letters
    if (left < 0) return Infinity;
    // stop incrementing the count of the string when the end is reached
    if (start >= length) return 0;
    // if the current character is the same as the last character added, it's always best to
    // add it and compress it
    if (s[start] === last) {
      return compressedLength(start + 1, last, count + 1, left) + Number(lengthCost.has(count));
    }
    // if the current character is different than the last character added
    return Math.min(
      // compare adding the new character to the string
      1 + compressedLength(start + 1, s[start], 1, left),
      // to deleting the new character from the string
      compressedLength(start + 1, last, count, left - 1),
    );
  };
  return compressedLength(0, '', 0, k);
};

getLengthOfOptimalCompression(
  'abcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxy',
  99,
);
