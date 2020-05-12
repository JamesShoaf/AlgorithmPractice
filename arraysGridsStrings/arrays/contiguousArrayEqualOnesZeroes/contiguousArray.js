// Given a binary array, find the maximum length of a contiguous subarray
// with equal number of 0 and 1.

const contiguousArray = (array) => {
  // calculate the longest possible frame size
  // reduce the array to a sum of values
  // return 0 immediately if the result is not an integer (invalid input array)
  const arraySum = array.reduce((acc, val) => acc + val, 0);
  if (!Number.isInteger(arraySum)) {
    return 0;
  }
  // otherwise, the longest possible array must be equal parts 1s and 0s, so it must be
  // the lesser of either 2 * arraySum or 2 * (array.length - arraySum)
  let currentFrame = 2 * Math.min(arraySum, array.length - arraySum);
  let targetSum = currentFrame / 2;
  // scan through each frame in decreasing size until a frame
  // with an equal number of 1s and 0s is found. Return the size of this frame.
  while (currentFrame > 0) {
    // check the sum of the first frame
    let initialSum = array.slice(0, currentFrame).reduce((acc, val) => acc + val, 0);
    if (initialSum === targetSum) {
      return currentFrame;
    }
    // shift frame by subtracting off first bit and adding on next bit,
    // then check again for the target
    for (let end = currentFrame, start = 0; end < array.length; end += 1, start += 1) {
      initialSum += array[end];
      initialSum -= array[start];
      if (initialSum === targetSum) {
        return currentFrame;
      }
    }
    // if no contiguous array is found, decrease the frame size and target value, then try again
    currentFrame -= 2;
    targetSum -= 1;
  }
  return 0;
};

module.exports = contiguousArray;
