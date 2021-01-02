/*
Given an array of integers nums.

A pair (i,j) is called good if nums[i] == nums[j] and i < j.

Return the number of good pairs.
*/

const goodPairs = (nums) => {
  let count = 0;
  const { length } = nums;
  for (let i = 0; i < length; i += 1) {
    for (let j = i + 1; j < length; j += 1) {
      if (nums[i] === nums[j]) count += 1;
    }
  }
  return count;
};

module.exports = { goodPairs };
