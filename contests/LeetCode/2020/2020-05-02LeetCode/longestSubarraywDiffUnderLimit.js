/*
Given an array of integers nums and an integer limit, return the size of the longest continuous
subarray such that the absolute difference between any two elements is less than or equal to limit.

In case there is no subarray satisfying the given condition return 0.
*/

// maximum holds length of longest array
// left pointer holds current start
// right pointer holds current end
// store max and min each time a left point has been found
// advance right pointer
// check against max and min
// if good, advance right pointer again and increment maximum if longer
// otherwise, reset max and min
// advance left pointer and see if a set of left and right could be made
// iterate between left and right for new local max and min

const longestSubarray = (nums, limit) => {
  if (nums.length === 0) return 0;
  let left = 0;
  let maximum = 1;
  let subsetMax = nums[0];
  let subsetMin = nums[0];
  for (let right = 1; right < nums.length; right += 1) {
    const rightVal = nums[right];
    if (rightVal > subsetMax) subsetMax = rightVal;
    if (rightVal < subsetMin) subsetMin = rightVal;
    if (subsetMax - subsetMin <= limit) {
      maximum = Math.max(maximum, right - left + 1);
    } else {
      while (left <= right && subsetMax - subsetMin > limit) {
        left += 1;
        const leftVal = nums[left];
        if (Math.abs(rightVal - leftVal) <= limit) {
          const currentsubArray = nums.slice(left, right + 1);
          subsetMax = Math.max(...currentsubArray);
          subsetMin = Math.min(...currentsubArray);
        }
      }
    }
  }
  return maximum;
};

module.exports = {
  longestSubarray,
};
