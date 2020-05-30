/*
You are given a sorted array consisting of only integers where every element appears exactly twice,
except for one element which appears exactly once. Find this single element that appears only once.

* Note: Your solution should run in O(log n) time and O(1) space.
*/

const singleNonDuplicate = (nums) => {
  const leftMismatch = (index) => nums[index] !== nums[index - 1];
  // edge case: last index is the outlier
  if (leftMismatch(nums.length - 1)) return nums[nums.length - 1];
  // search only odd indices
  let left = 1;
  let right = nums.length - 2;
  while (right !== left) {
    let mid = (left + right) / 2;
    // if mid is not odd, move it left
    if (mid % 2 === 0) mid -= 1;
    // if the even index left of mid is not a match, mid is to the right of the nonduplicate
    if (leftMismatch(mid)) right = mid;
    // because of left justification, if the insertion is in the middle, left will not reach right
    else if (left === mid) return nums[mid + 1];
    // otherwise move left to mid
    else left = mid;
  }
  return nums[left - 1];
};

module.exports = {
  singleNonDuplicate,
};
