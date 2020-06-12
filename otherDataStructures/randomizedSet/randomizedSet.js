/*
Design a data structure that supports all following operations in average O(1) time.

    insert(val): Inserts an item val to the set if not already present.
    remove(val): Removes an item val from the set if present.
    getRandom: Returns a random element from current set of elements. Each element must have the
      same probability of being returned.

*/

class RandomizedSet {
  constructor() {
    this.set = {};
    this.array = [];
  }

  insert(val) {
    const { set, array } = this;
    // if the value is already present in the set, return false
    if (set[val] !== undefined) return false;
    // if the value is not in the set, add the last array index and value to the set
    set[val] = array.length;
    // and push the value to the array
    array.push(val);
    return true;
  }

  remove(val) {
    const { set, array } = this;
    // if the value is not in the set, return false
    if (set[val] === undefined) return false;
    // if the value is in the set, get the array index of the value from the set
    const oldIndex = set[val];
    if (array.length > 1) {
      // swap the value with the value at the last index in the set
      const lastIndex = array.length - 1;
      [array[lastIndex], array[oldIndex]] = [array[oldIndex], array[lastIndex]];
      // update the index for the value that was at the last index
      set[array[oldIndex]] = oldIndex;
    }
    // pop off the last value of the array (the current value)
    array.pop();
    // delete the value's key from the set
    delete set[val];
    // return true
    return true;
  }

  getRandom() {
    const { array } = this;
    return array[Math.floor(Math.random() * array.length)];
  }
}

module.exports = { RandomizedSet };
