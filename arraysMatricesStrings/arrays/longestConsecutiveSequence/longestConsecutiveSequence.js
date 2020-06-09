/*
Given an unsorted array of integers, find the length of the longest consecutive elements sequence.
*/

const longestConsecutiveSequence = (nums) => {
  const { length } = nums;
  if (length < 2) return length;
  const [first] = nums;
  const set = new Set();
  let highest = first;
  let lowest = first;
  for (let i = 0; i < length; i += 1) {
    const currentInt = nums[i];
    if (currentInt > highest) highest = currentInt;
    if (currentInt < lowest) lowest = currentInt;
    set.add(currentInt);
  }
  let last = lowest;
  let longestRun = 1;
  let currentRun = 1;
  for (let i = lowest + 1; i <= highest; i += 1) {
    if (set.has(i)) {
      if (last === i - 1) {
        currentRun += 1;
        if (currentRun > longestRun) longestRun = currentRun;
      }
      if (last !== i - 1) {
        currentRun = 1;
      }
      last = i;
    }
  }
  return longestRun;
};

module.exports = { longestConsecutiveSequence };
