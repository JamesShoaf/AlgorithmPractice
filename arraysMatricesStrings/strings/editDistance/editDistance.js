/*
Given two words word1 and word2, find the minimum number of operations required to convert
word1 to word2.

You have the following 3 operations permitted on a word:

    Insert a character
    Delete a character
    Replace a character
*/

const editDistance = (word1, word2) => {
  const distance = (shortWord, longWord) => {
    const [shortLength, longLength] = [shortWord.length, longWord.length];
    // create a 2 x shortLength matrix for storing values.
    const dp = [...Array(2)].map(() => Array(shortLength + 1));
    // initialize the first row to 0s and the second row to the column index
    // the first row represents additions to convert an empty string to the short string up to
    // that index
    for (let i = 0; i <= shortLength; i += 1) {
      dp[0][i] = 0;
      dp[1][i] = i;
    }
    // for each character in long
    for (let long = 0; long < longLength; long += 1) {
      // for each character in short (1-indexed)
      for (let short = 0; short <= shortLength; short += 1) {
        // the first column represents additions to convert an empty string to the long string
        // up to the long index
        if (short === 0) dp[long % 2][0] = dp[(long + 1) % 2][0] + 1;
        if (short !== 0) {
          if (longWord[long] === shortWord[short - 1]) {
            dp[long % 2][short] = dp[(long + 1) % 2][short - 1];
          }
          if (longWord[long] !== shortWord[short - 1]) {
            dp[long % 2][short] = Math.min(
              dp[(long + 1) % 2][short - 1],
              dp[(long + 1) % 2][short],
              dp[long % 2][short - 1],
            ) + 1;
          }
        }
      }
    }
    return dp[(longLength + 1) % 2][shortLength];
  };
  return (word1.length > word2.length)
    ? distance(word2, word1)
    : distance(word1, word2);
};

module.exports = { editDistance };
