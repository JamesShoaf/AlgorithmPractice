const { PriorityQueue } = require('../../heaps/priorityQueue');

/* You are given an undirected weighted graph of n nodes (0-indexed), represented by an edge list
where edges[i] = [a, b] is an undirected edge connecting the nodes a and b with a probability of
success of traversing that edge succProb[i].

Given two nodes start and end, find the path with the maximum probability of success to go from
start to end and return its success probability.

If there is no path from start to end, return 0. Your answer will be accepted if it differs from
the correct answer by at most 1e-5. */

// variation on Djikstra's using high prob instead of min distance

const maxProbability = (n, edges, succProb, start, end) => {
  // max heap
  const pq = new PriorityQueue((a, b) => a[0] > b[0]);
  const visited = new Set();
  // origin at index, key:value = destination:probability
  const nodes = [...Array(n)].map(() => ({}));
  const { length: numEdges } = edges;
  for (let i = 0; i < numEdges; i += 1) {
    const [origin, destination] = edges[i];
    const probability = succProb[i];
    nodes[origin][destination] = probability;
    nodes[destination][origin] = probability;
  }

  // 1.0 probability to reach start from start
  pq.push([1, start]);

  while (!pq.isEmpty()) {
    const [probability, origin] = pq.pop();
    // when the end node is reached, return the probability
    if (origin === end) return probability;
    visited.add(origin);
    const originNode = nodes[origin];
    for (const destination in originNode) {
      if (!visited.has(Number(destination))) {
        pq.push([originNode[destination] * probability, Number(destination)]);
      }
    }
  }

  // if the end node is not connected, the queue will empty before it is reached
  return 0;
};

module.exports = { maxProbability };
