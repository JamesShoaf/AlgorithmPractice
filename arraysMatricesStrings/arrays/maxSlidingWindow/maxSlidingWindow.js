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
    this.higher = null;
    this.lower = null;
    this.older = null;
    this.newer = null;
  }
}

const maxSlidingWindow = (nums, k) => {
  const { length } = nums;
  if (k === 0 || length === 0 || k > length) return [];
  const maxWindow = [];
  const firstNode = new QueueNode(nums[0], 0);
  const nodes = [firstNode];
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
    nodes.push(current);
    newest.newer = current;
    current.older = newest;
    newest = current;
    // if the value of the new node is greater than or equal to the current highest,
    if (int >= highest.val) {
      // set the highest node's high pointer to the new node
      highest.higher = current;
      current.lower = highest;
      // and remove all lower previous nodes
      while (int >= highest.val && highest !== current) {
        // set a temporary pointer to the highest node
        const toDelete = highest;
        // reassign the pointer to the oldest node
        if (oldest === highest) oldest = oldest.newer;
        // reassign the pointers to the highest and lowest node
        if (highest === lowest) {
          lowest = current;
          highest = current;
        } else {
          highest = highest.lower;
        }
        // delete the previous highest node
        toDelete.delete();
      }
      // and set the current node as the new highest
      highest = current;
    }
    // if the node is less than the current highest node, add it as the lower of the previous lowest
    if (int < highest.val) {
      lowest.lower = current;
      current.higher = lowest;
      // if the newest node's value is greater than the lowest node's value, remove the lowest
      while (int >= lowest.val) {
        const toDelete = lowest;
        lowest = lowest.higher;
        toDelete.delete();
      }
      lowest = current;
    }
    // next, remove the oldest node if its index is outside the frame
    while (i >= oldest.index + k) {
      const toDelete = oldest;
      // reassign the highest
      if (highest === oldest) highest = highest.lower;
      // reassign the oldest
      oldest = oldest.newer;
      // and delete the previous oldest
      toDelete.delete();
    }
    // finally, push the highest node's value to the output
    if (i >= k - 1) maxWindow.push(highest.val);
  }
  // return output
  return maxWindow;
};

module.exports = { maxSlidingWindow };
