/* Given an array of numbers arr. A sequence of numbers is called an arithmetic progression
if the difference between any two consecutive elements is the same.

Return true if the array can be rearranged to form an arithmetic progression, otherwise,
return false. */

const canMakeArithmeticProgression = (arr) => {
  if (!Array.isArray(arr)) return false;
  const { length } = arr;
  if (length < 2) return true;
  const sorted = arr.sort((a, b) => a - b);
  const diff = sorted[1] - sorted[0];
  for (let i = 2; i < length; i += 1) {
    if (sorted[i] - sorted[i - 1] !== diff) return false;
  }
  return true;
};
