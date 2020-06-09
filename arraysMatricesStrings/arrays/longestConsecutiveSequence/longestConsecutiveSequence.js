/*
Given an unsorted array of integers, find the length of the longest consecutive elements sequence.
*/

const longestConsecutiveSequence = (nums) => {
  // add each integer to a set
  const set = new Set(nums);

  const { length } = nums;
  let longestRun = 0;
  // for each integer in the array
  for (let i = 0; i < length; i += 1) {
    const currentInt = nums[i];
    // for each start of a run (n - 1 not in the set)
    if (!set.has(currentInt - 1)) {
      // iterate through sequential numbers to find how many are in the current run
      let currentRun = 1;
      for (let j = currentInt; set.has(j + 1); j += 1) currentRun += 1;
      // then update the longest run
      if (currentRun > longestRun) longestRun = currentRun;
    }
  }
  // return the longest run
  return longestRun;
};

module.exports = { longestConsecutiveSequence };
