/*
Given an array w of positive integers, where w[i] describes the weight of index i, write a function
pickIndex which randomly picks an index in proportion to its weight.

Note:

    1 <= w.length <= 10000
    1 <= w[i] <= 10^5
    pickIndex will be called at most 10000 times.

*/

class WeightedIndex {
  constructor(nums) {
    let runningTotal = 0;
    this.weights = nums.map((int) => {
      runningTotal += int;
      return runningTotal;
    });
  }

  pickIndex() {
    const { weights } = this;
    let high = weights.length - 1;
    const max = weights[high];
    let low = 0;
    // choose a number between 1 and the max
    const random = Math.random() * max;
    // binary search weights array for the matching value
    while (high !== low) {
      const mid = Math.floor((high + low) / 2);
      // edge case
      const midWeight = weights[mid];
      if (midWeight === random) return mid;
      if (midWeight > random) {
        if (mid === low) return low;
        high = mid;
      }
      if (midWeight < random) {
        if (mid === low) return high;
        low = mid;
      }
    }
    return high;
  }
}

module.exports = { WeightedIndex };
