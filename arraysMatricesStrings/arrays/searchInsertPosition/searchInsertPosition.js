/*
Given a sorted array and a target value, return the index if the target is found. If not, return
the index where it would be if it were inserted in order.

You may assume no duplicates in the array.
*/

const searchInsertPosition = (nums, target) => {
  let low = 0;
  let high = nums.length;
  while (low !== high) {
    const mid = Math.floor((low + high) / 2);
    if (nums[mid] === target) return mid;
    if (nums[mid] > target) {
      if (low === mid) return low;
      high = mid;
    }
    if (nums[mid] < target) {
      if (low === mid) return low + 1;
      low = mid;
    }
  }
  // if nums is empty, return 0
  return low;
};

module.exports = { searchInsertPosition };
