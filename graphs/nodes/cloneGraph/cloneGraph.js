/*
Given a reference of a node in a connected undirected graph.

Return a deep copy (clone) of the graph.

Each node in the graph contains a val (int) and a list (List[Node]) of its neighbors.

Constraints:

    1 <= Node.val <= 100
    Node.val is unique for each node.
    Number of Nodes will not exceed 100.
    There is no repeated edges and no self-loops in the graph.
    The Graph is connected and all nodes can be visited starting from the given node.

*/

class Node {
  constructor(val = 0, neighbors = []) {
    this.val = val;
    this.neighbors = neighbors;
  }
}

const cloneGraph = (node) => {
  if (!node) return node;
  const map = {};
  const set = new Set();
  const dfs = (inner) => {
    if (set.has(inner.val)) return;
    set.add(inner.val);
    map[inner.val] = new Node(inner.val);
    inner.neighbors.forEach((neighbor) => {
      dfs(neighbor, map, set);
      map[inner.val].neighbors.push(map[neighbor.val]);
    });
  };
  dfs(node, map, set);
  return map[node.val];
};

/*
#1 solution for comparison
  var cloneGraph = function(node) {
      if (!node) return node;
      return collectAllNodes(node).get(node.val);
  };

  function collectAllNodes(node, results = new Map()) {

    if (!results.has(node.val)) {
          let cloned = new Node(node.val);
          results.set(node.val, cloned);

          for (let neighbor of node.neighbors) {
              collectAllNodes(neighbor, results);
              cloned.neighbors.push(results.get(neighbor.val))
          }
      }

      return results;

  }
*/

module.exports = { cloneGraph };
