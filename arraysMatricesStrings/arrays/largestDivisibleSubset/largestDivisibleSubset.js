/*
Given a set of distinct positive integers, find the largest subset such that every pair (Si, Sj)
of elements in this subset satisfies:

Si % Sj = 0 or Sj % Si = 0.

If there are multiple solutions, return one of them.
*/

const largestDivisibleSubset = (nums) => {
  // sort the array
  const sorted = nums.sort((a, b) => a - b);
  const { length } = sorted;
  // create a map of longest paths
  const pathMap = {};
  let bestPath = [];
  let bestLength = 0;

  // when a best path is found, recursively build that path to output it
  const recursivePathBuilder = (val) => {
    const [prefix] = pathMap[val];
    return (prefix)
      ? [...recursivePathBuilder(prefix), val]
      : [val];
  };

  // for each element, trace backwards for smaller elements such that element % smaller = 0
  for (let i = 0; i < length; i += 1) {
    const current = sorted[i];
    // if there are no smaller elements, the path is the element by itself
    let prefix = null;
    let longestPrefixLength = 0;
    // find which of the smaller elements has the longest path to reach it
    // (in case of a tie, choose the larger one)
    for (let j = i - 1; j >= 0; j -= 1) {
      const smaller = sorted[j];
      if (current % smaller === 0) {
        const prefixLength = pathMap[smaller][1];
        if (prefixLength > longestPrefixLength) {
          longestPrefixLength = prefixLength;
          prefix = smaller;
        }
      }
    }
    // store the smaller element and the length of the path to the current element
    pathMap[current] = [prefix, longestPrefixLength + 1];
    // if the length of the path to the current element is the longest seen, updateBestpath
    if (bestLength <= longestPrefixLength) {
      bestPath = recursivePathBuilder(current);
      bestLength = longestPrefixLength + 1;
    }
  }
  // after all elements have been mapped, return the longest path
  return bestPath;
};

module.exports = { largestDivisibleSubset };
