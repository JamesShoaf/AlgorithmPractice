/*
Given an array of non-negative integers, you are initially positioned at the
first index of the array.
Each element in the array represents your maximum jump length at that position.
Determine if you are able to reach the last index.
*/

const jumpGame = (array) => {
  const lastIndex = array.length - 1;
  const lookAhead = (startIndex) => {
    const moveOptions = array[startIndex];
    if (lastIndex <= startIndex + moveOptions) {
      return true;
    }
    let nextIndex = startIndex;
    let farthestForward = 0;
    for (let i = 0; i < moveOptions; i += 1) {
      const currentIndex = startIndex + i;
      const forward = currentIndex + array[currentIndex];
      if (farthestForward <= forward) {
        farthestForward = forward;
        nextIndex = currentIndex;
      }
    }
    if (nextIndex === startIndex) {
      return false;
    }
    return lookAhead(nextIndex);
  };
  return lookAhead(0);
};

module.exports = {
  jumpGame,
};
