// Given an unsorted integer array, find the smallest missing positive integer.
// Runs in O(n) time and uses constant * extra * space.

const firstMissingPositive = (nums) => {
  const { length } = nums;
  // convert all non-positive integers to length + 1
  // [0, -1, 5, 1, 4] --> [6, 6, 5, 1, 4]
  for (let i = 0; i < length; i += 1) {
    const curr = nums[i];
    if (curr < 1) nums[i] = length + 1;
  }
  // for each index, if the absolute value of that index is less than the length of the array
  // set the sign of the integer at that index negative
  // [6, 6, 5, 1, 4] --> [-6, 6, 5, -1, -4]
  for (let i = 0; i < length; i += 1) {
    const curr = Math.abs(nums[i]);
    if (curr <= length) {
      nums[curr - 1] = -Math.abs(nums[curr - 1]);
    }
  }
  // finally, iterate through the array until a positive index is found, then return that index
  let missing = 0;
  while (nums[missing] < 0) {
    missing += 1;
  }
  // if the array is empty, return 1
  return missing + 1;
};

module.exports = { firstMissingPositive };
