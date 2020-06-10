/*
Given an array nums, there is a sliding window of size k which is moving from the very left of the
array to the very right. You can only see the k numbers in the window. Each time the sliding window
moves right by one position. Return the max sliding window.
*/

// doubly linked node
// stores pointers to next higher, next lower, next older, and next newer node
// stores integer value
// deletes self if needed by linking old/new and high/low

class QueueNode {
  constructor(value, index) {
    this.val = value;
    this.index = index;
    this.higher = null;
    this.lower = null;
    this.older = null;
    this.newer = null;
  }

  delete() {
    const {
      higher,
      lower,
      older,
      newer,
    } = this;
    if (higher) higher.lower = lower;
    if (lower) lower.higher = higher;
    if (older) older.newer = newer;
    if (newer) newer.older = older;
  }
}

const maxSlidingWindow = (nums, k) => {
  const { length } = nums;
  if (k === 0 || length === 0 || k > length) return [];
  const maxWindow = [];
  const firstNode = new QueueNode(nums[0], 0);
  let highest = firstNode;
  let lowest = firstNode;
  let oldest = firstNode;
  let newest = firstNode;
  if (k === 1) maxWindow.push(firstNode.val);
  // for each integer in the array
  for (let i = 1; i < length; i += 1) {
    // create a new node for the integer and add it as the newest node
    const int = nums[i];
    const current = new QueueNode(int, i);
    newest.newer = current;
    current.older = newest;
    newest = current;
    // if the value of the new node is greater than or equal to the current highest,
    // set the highest node's high pointer to the new node, and set the highest
    // pointer to the new node.
    if (int >= highest.val) {
      highest.higher = current;
      current.lower = highest;
      highest = current;
    }
    // if the node is less than the current highest node, add it as the lower of the previous lowest
    // if the node's value is greater than the previous least node's value, remove that node
    if (int < highest.val) {
      lowest.lower = current;
      current.higher = lowest;
      while (int >= lowest.val) {
        const toDelete = lowest;
        lowest = lowest.higher;
        toDelete.delete();
      }
      lowest = current;
    }
    // if the oldest node is outside the frame
    while (i >= oldest.index + k) {
      const toDelete = oldest;
      // if the oldest node is the highest, set the highest to that node's lower neighbor
      if (highest === oldest) highest = highest.lower;
      // also fix the pointer to the lowest node
      if (lowest === oldest) lowest = lowest.higher;
      // in either case, remove the old node and update the reference to the oldest node
      oldest = oldest.newer;
      toDelete.delete();
    }
    // finally, push the highest node's value to the output
    if (i >= k - 1) maxWindow.push(highest.val);
  }
  // return output
  return maxWindow;
};

// const test = [
//   -95, 92, -85, 59, -59, -14, 88, -39, 2, 92, 94, 79, 78, -58, 37, 48, 63, -91, 91, 74, -28, 39,
//   90, -9, -72, -88, -72, 93, 38, 14, -83, -2, 21, 4, -75, -65, 3, 63, 100, 59, -48, 43, 35, -49,
//   48, -36, -64, -13, -7, -29, 87, 34, 56, -39, -5, -27, -28, 10, -57, 100, -43, -98, 19, -59,
//   78, -28, -91, 67, 41, -64, 76, 5, -58, -89, 83, 26, -7, -82, -32, -76, 86, 52, -6, 84, 20, 51,
//   -86, 26, 46, 35, -23, 30, -51, 54, 19, 30, 27, 80, 45, 22,
// ];
// const windowSize = 10;
// const arr = maxSlidingWindow(test, windowSize);
// console.log(arr);

module.exports = { maxSlidingWindow };
