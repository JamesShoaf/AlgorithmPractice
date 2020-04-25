/*
Given an array of non-negative integers, you are initially positioned at the
first index of the array.
Each element in the array represents your maximum jump length at that position.
Determine if you are able to reach the last index.
*/

const jumpGame = (array) => {
  const lastIndex = array.length - 1;
  const recursiveJumpChecker = (endpoint) => {
    if (endpoint === 0) {
      return true;
    }
    let furthestBack;
    for (let i = endpoint - 1; i >= 0; i -= 1) {
      const currentIndex = i;
      if (array[currentIndex] >= endpoint - currentIndex) {
        furthestBack = currentIndex;
      }
    }
    if (furthestBack === undefined) {
      return false;
    }
    return recursiveJumpChecker(furthestBack);
  };
  return recursiveJumpChecker(lastIndex);
};

module.exports = {
  jumpGame,
};
