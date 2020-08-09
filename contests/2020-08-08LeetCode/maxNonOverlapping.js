/* 
Given an array nums and an integer target.

Return the maximum number of non-empty non-overlapping subarrays such that the sum of values in each subarray is equal to target.
*/

const maxNonOverlapping = (nums, target) => {
  let set = new Set();
  let runningTotal = 0;
  let count = 0;
  const { length } = nums;
  for (let i = 0; i < length; i += 1) {
    runningTotal += nums[i];
    if (runningTotal === target || set.has(runningTotal - target)) {
      count += 1;
      set = new Set();
      runningTotal = 0;
    } else {
      set.add(runningTotal);
    }
  }
  return count;
};
