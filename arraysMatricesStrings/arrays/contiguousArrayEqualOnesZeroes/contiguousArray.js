// Given a binary array, find the maximum length of a contiguous subarray
// with equal number of 0 and 1.

const contiguousArray = (nums) => {
  let runningTotal = 0;
  const totalMap = { 0: [-1, -1] };
  let maxSize = 0;
  const { length } = nums;
  // add 1 to runningTotal at each 1, and subtract 1 at each 0
  for (let i = 0; i < length; i += 1) {
    const currentNum = nums[i];
    switch (currentNum) {
      case 1:
        runningTotal += 1;
        break;
      case 0:
        runningTotal -= 1;
        break;
      default:
        return 0; // array is not a valid binary array, return 0
    }
    // store the first index and last index with a given total
    // these indices both have the same number of unmatched 1s and 0s
    if (!totalMap[runningTotal]) {
      totalMap[runningTotal] = [i, i];
    }
    const currentScore = totalMap[runningTotal];
    currentScore[1] = i;
    // the maximum size of matched 1s and 0s is the largest difference between indices
    // at any runningTotal
    maxSize = Math.max(maxSize, i - currentScore[0]);
  }
  return maxSize;
};

module.exports = contiguousArray;
