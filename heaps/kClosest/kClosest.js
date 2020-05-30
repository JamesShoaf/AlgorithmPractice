/*
We have a list of points on the plane.  Find the K closest points to the origin (0, 0).

(Here, the distance between two points on a plane is the Euclidean distance.)

You may return the answer in any order.  The answer is guaranteed to be unique (except for the
  order that it is in.)
*/

const { PriorityQueue } = require('../priorityQueue');

const kClosest = (points, K) => {
  // create array of [distance, [x, y]]
  const queueElements = points.map(([x, y]) => {
    const distance = Math.sqrt(x ** 2 + y ** 2);
    return [distance, [x, y]];
  });
  // add comparator for shortest distance
  const distanceQueue = new PriorityQueue((a, b) => a[0] < b[0]);
  // queue up all elements
  distanceQueue.push(...queueElements);
  const output = [];
  // then dequeue the first K of them
  for (let i = 0; i < K; i += 1) {
    output.push(distanceQueue.pop()[1]);
  }
  return output;
};

module.exports = { kClosest };
