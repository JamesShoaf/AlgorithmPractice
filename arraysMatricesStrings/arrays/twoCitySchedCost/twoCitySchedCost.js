const { PriorityQueue } = require('../../../heaps/priorityQueue');

const twoCitySchedCost = (costs) => {
  const { length } = costs;
  const n = length / 2;
  if (!Number.isInteger(n)) throw Error('costs.length must be even');
  // maxHeap of differences a - b
  const pq = new PriorityQueue();
  let sum = 0;
  // iterate through costs
  for (let i = 0; i < length; i += 1) {
    const [a, b] = costs[i];
    if (typeof a !== 'number' || typeof b !== 'number') throw Error(`invalid cost at index ${i}`);
    // sum the cost of sending all persons to a
    sum += a;
    // and add the difference a - b to the heap
    pq.push(a - b);
  }
  // then switch the n most relatively expensive a routes to b
  // a - (a - b) = b
  for (let i = 0; i < n; i += 1) sum -= pq.pop();
  return sum;
};

module.exports = { twoCitySchedCost };
