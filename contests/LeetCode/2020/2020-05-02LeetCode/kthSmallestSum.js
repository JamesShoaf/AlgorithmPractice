/*
You are given an m * n matrix, mat, and an integer k, which has its rows sorted in
non-decreasing order.

You are allowed to choose exactly 1 element from each row to form an array. Return
the Kth smallest array sum among all possible arrays.

Example 1:

Input: mat = [[1,3,11],[2,4,6]], k = 5
Output: 7
Explanation: Choosing one element from each row, the first k smallest sum are:
[1,2], [1,4], [3,2], [3,4], [1,6]. Where the 5th sum is 7.
Example 2:

Input: mat = [[1,3,11],[2,4,6]], k = 9
Output: 17
Example 3:

Input: mat = [[1,10,10],[1,4,5],[2,3,6]], k = 7
Output: 9
Explanation: Choosing one element from each row, the first k smallest sum are:
[1,1,2], [1,1,3], [1,4,2], [1,4,3], [1,1,6], [1,5,2], [1,5,3]. Where the 7th sum is 9.
Example 4:

Input: mat = [[1,1,10],[2,2,9]], k = 7
Output: 12

Constraints:

m == mat.length
n == mat.length[i]
1 <= m, n <= 40
1 <= k <= min(200, n ^ m)
1 <= mat[i][j] <= 5000
mat[i] is a non decreasing array.
*/

class PrioritySetQueue {
  constructor() {
    this.items = {};
    this.dequeued = new Set();
  }

  enqueue(priority, value) {
    if (!this.dequeued.has(value)) {
      if (this.items[priority] === undefined) {
        this.items[priority] = new Set();
      }
      this.items[priority].add(value);
    }
  }

  dequeue() {
    let keys = Object.keys(this.items);
    keys = keys.map((key) => parseInt(key, 10)).sort((a, b) => a - b);
    const k = keys[0];
    const val = this.items[`${k}`].entries().next().value[0];
    this.items[`${k}`].delete(val);
    if (this.items[`${k}`].size === 0) delete this.items[`${k}`];
    this.dequeued.add(val);
    return [k, val];
  }
}

const kthSmallestSum = (mat, k) => {
  const rows = mat.length;
  const smallest = [];
  smallest.length = rows;
  smallest.fill(0);
  const arraySum = (array) => mat
    .map((row, index) => row[array[index]])
    .reduce((acc, val) => acc + val);
  const queue = new PrioritySetQueue();
  queue.enqueue(arraySum(smallest), JSON.stringify(smallest));
  for (let i = 1; i < k; i += 1) {
    const [currentSum, arrayString] = queue.dequeue();
    const array = JSON.parse(arrayString);
    array.forEach((val, index) => {
      const incrementSlice = array.slice();
      const newVal = mat[index][val + 1];
      if (newVal !== undefined) {
        const newSum = currentSum + newVal - mat[index][val];
        incrementSlice[index] = val + 1;
        queue.enqueue(newSum, JSON.stringify(incrementSlice));
      }
    });
  }
  return queue.dequeue()[0];
};

module.exports = {
  kthSmallestSum,
};
