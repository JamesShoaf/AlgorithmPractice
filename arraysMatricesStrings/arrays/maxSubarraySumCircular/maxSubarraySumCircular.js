const maxSubarraySumCircular = (nums) => {
  const { length } = nums;
  let worstSum = nums[0];
  let currentWorst = nums[0];
  let bestSum = nums[0];
  let currentBest = nums[0];
  let straightSum = nums[0];
  for (let i = 1; i < length; i += 1) {
    straightSum += nums[i];
    if (currentBest < 0) currentBest = nums[i];
    else currentBest += nums[i];
    if (currentWorst > 0) currentWorst = nums[i];
    else currentWorst += nums[i];
    if (currentBest > bestSum) bestSum = currentBest;
    if (currentWorst < worstSum) worstSum = currentWorst;
  }
  if (straightSum === worstSum) return bestSum;
  return Math.max(bestSum, straightSum - worstSum);
};

module.exports = { maxSubarraySumCircular };
