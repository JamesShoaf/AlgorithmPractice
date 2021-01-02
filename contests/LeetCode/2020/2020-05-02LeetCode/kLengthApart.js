/*
Given an array nums of 0s and 1s and an integer k, return True if all 1's are at least k places
away from each other, otherwise return False.
*/

const kLengthApart = (nums, k) => {
  let lastOne;
  for (let i = 0; i < nums.length; i += 1) {
    if (nums[i] === 1) {
      lastOne = i;
      break;
    }
  }
  if (lastOne === undefined) return true;
  for (let i = lastOne + 1; i < nums.length; i += 1) {
    if (nums[i] === 1) {
      if (i - lastOne <= k) return false;
      lastOne = i;
    }
  }
  return true;
};

module.exports = {
  kLengthApart,
};
