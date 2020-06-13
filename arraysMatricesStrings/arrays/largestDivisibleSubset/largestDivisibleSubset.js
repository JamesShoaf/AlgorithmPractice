/*
Given a set of distinct positive integers, find the largest subset such that every pair (Si, Sj)
of elements in this subset satisfies:

Si % Sj = 0 or Sj % Si = 0.

If there are multiple solutions, return one of them.
*/


const largestDivisibleSubset = (nums) => {
  // sort the array
  // initialize a set of visited integers
  const stack = [];
  let bestStack = [];
  let bestLength = 0;
  const sorted = nums.sort((a, b) => a - b);
  const { length } = sorted;
  const visited = new Set();

  // for each element in the array
  for (let i = 0; i < length; i += 1) {
    const current = sorted[i];
    // if the element has not been visited, add it to the stack
    if (!visited.has(current)) stack.push([current, i]);
    // while the stack is not empty
    while (stack.length) {
      const [biggest, lastIndex] = stack[stack.length - 1];
      // for each following element
      for (let j = lastIndex + 1; j < length; j += 1) {
        const next = sorted[j];
        // if the element has already been visited, skip that element
        // add the next element to the stack if element % stack[stack.length - 1] = 0
        // and restart the loop
        if (!visited.has(next) && next % biggest === 0) {
          stack.push([next, j]);
          break;
        }
      }
      if (stack[stack.length - 1][0] === biggest) {
        // if the last element is reached, compare the length of the stack to the
        // longest stack length
        if (stack.length > bestLength) {
          // overwrite the best stack and stack length if the current stack is longer
          bestStack = stack.map(([val]) => val);
          bestLength = stack.length;
        }
        // pop the last element off the stack and backtrack to the previous node
        visited.add(stack.pop()[0]);
      }
    }
  }
  // once all nodes have been visited, return the longest stack
  return bestStack;
};

module.exports = { largestDivisibleSubset };
