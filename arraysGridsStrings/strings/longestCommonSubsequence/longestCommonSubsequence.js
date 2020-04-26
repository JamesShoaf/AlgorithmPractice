// Given two strings text1 and text2, return the length of their longest common subsequence.

// A subsequence of a string is a new string generated from the original string with some
// characters(can be none) deleted without changing the relative order of the remaining characters.
// (eg, "ace" is a subsequence of "abcde" while "aec" is not). A common subsequence of two strings
// is a subsequence that is common to both strings.

// If there is no common subsequence, return 0.

const longestCommonSubsequence = (string1, string2) => {
  // memoize helper function
  const memoize = (func) => {
    const cache = {};
    const memoized = (...args) => {
      const argString = JSON.stringify(args);
      if (cache[argString] !== undefined) {
        return cache[argString];
      }
      const results = func(...args);
      cache[argString] = results;
      return results;
    };
    return memoized;
  };

  let dynamic;

  const dynamizer = (index1, index2) => {
    if (dynamic === undefined) {
      dynamic = memoize((ind1, ind2) => {
        // if text1[i] == text2[j]
        if (string1[ind1] === string2[ind2]) {
          // DP[i][j] = DP[i - 1][j - 1] + 1 ,
          return dynamizer(ind1 - 1, ind2 - 1) + 1;
        }
        // else
        // DP[i][j] = max(DP[i - 1][j], DP[i][j - 1])
        return Math.max(dynamizer(ind1 - 1, ind2), dynamizer(ind1, ind2 - 1));
      });
    }
    if (index1 === -1 || index2 === -1) {
      return 0;
    }
    return dynamic(index1, index2);
  };

  return dynamizer(string1.length - 1, string2.length - 1);
};

module.exports = {
  longestCommonSubsequence,
};
