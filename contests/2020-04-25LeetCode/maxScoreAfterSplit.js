// Given a string s of zeros and ones, return the maximum score after splitting the string
// into two non-empty substrings (i.e. left substring and right substring).

// The score after splitting a string is the number of zeros in the left substring plus
// the number of ones in the right substring.

const maxScoreAfterSplit = (string) => {
  let highNetScore = 0;
  let netScore = 0;
  let highNetScoreIndex = -1; // represents splitting after this index
  // scan through string
  // + 1 point for 0s
  // - 1 point for 1s (symbolic for cutting them off from the right)
  for (let i = 0; i < string.length; i += 1) {
    const currentChar = string[i];
    if (currentChar === '0') {
      netScore += 1;
      if (netScore >= highNetScore) {
        highNetScore = netScore;
        highNetScoreIndex = i;
      }
    }
    if (currentChar === '1') {
      netScore -= 1;
    }
  }
  const leftSlice = string.slice(0, highNetScoreIndex + 1);
  const rightSlice = string.slice(highNetScoreIndex + 1);
  let finalScore = 0;
  for (let i = 0; i < leftSlice.length; i += 1) {
    const currentChar = leftSlice[i];
    if (currentChar === '0') {
      finalScore += 1;
    }
  }
  for (let i = 0; i < rightSlice.length; i += 1) {
    const currentChar = rightSlice[i];
    if (currentChar === '1') {
      finalScore += 1;
    }
  }
  // find index with highest net score
  // score left string
  // score right string

  return finalScore;
};

module.exports = {
  maxScoreAfterSplit,
};
