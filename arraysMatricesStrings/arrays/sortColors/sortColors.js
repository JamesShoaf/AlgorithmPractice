/*
Given an array with n objects colored red, white or blue, sort them in-place so that objects of the
same color are adjacent, with the colors in the order red, white and blue.

Here, we will use the integers 0, 1, and 2 to represent the color red, white, and blue respectively.
*/

const sortColors = (nums) => {
  let zeroInsertIndex = 0;
  let oneInsertIndex = 0;
  const { length } = nums;
  for (let i = 0; i < length; i += 1) {
    if (nums[i] < 2) {
      // swap 0s to their insert index, and increment the number of zeroes
      if (nums[i] === 0) {
        [nums[zeroInsertIndex], nums[i]] = [nums[i], nums[zeroInsertIndex]];
        zeroInsertIndex += 1;
      }
      // if there was a 1 at the zero index, or if the current int is a 1, swap it to the one index
      if (nums[i] === 1) [nums[oneInsertIndex], nums[i]] = [nums[i], nums[oneInsertIndex]];
      // increment the one index in the case of either a 0 or a 1 at the current index
      oneInsertIndex += 1;
    }
  }
};

module.exports = { sortColors };
