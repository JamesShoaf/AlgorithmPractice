// Given 2 arrays of integers, return a boolean indicating whether a third array
// contains the union of those arrays in the same relative order

const orderChecker = (expected1, expected2, actual) => {
  if (actual.length !== expected1.length + expected2.length) {
    return false;
  }
  let i = (expected1.length === 0 ? null : 0);
  let j = (expected2.length === 0 ? null : 0);
  for (let k = 0; k < actual.length; k += 1) {
    if (i !== null && i < expected1.length) {
      if (actual[k] === expected1[i]) {
        i += 1;
      }
    }
    if (j !== null && j < expected2.length) {
      if (actual[k] === expected2[j]) {
        j += 1;
      }
    }
  }
  if (i < expected1.length || j < expected2.length) {
    return false;
  }
  return true;
};

module.exports = orderChecker;
