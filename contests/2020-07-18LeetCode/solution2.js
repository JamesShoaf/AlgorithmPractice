/*
Given a tree (i.e. a connected, undirected graph that has no cycles) consisting of n nodes numbered from 0 to n - 1 and exactly n - 1 edges. The root of the tree is the node 0, and each node of the tree has a label which is a lower-case character given in the string labels (i.e. The node with the number i has the label labels[i]).

The edges array is given on the form edges[i] = [ai, bi], which means there is an edge between nodes ai and bi in the tree.

Return an array of size n where ans[i] is the number of nodes in the subtree of the ith node which have the same label as node i.

A subtree of a tree T is the tree consisting of a node in T and all of its descendant nodes.
*/

class Tree {
  constructor(id, label = null) {
    this.id = id;
    this.label = label;
    this.children = [];
    this.subTreeLabels = {};
    this.subTreeLabels[label] = 1;
  }
}

const countSubTrees = (n, edges, labels) => {
  const nodes = [...Array(n)].map((_, index) => new Tree(index, labels[index]));
  const edgeCount = n - 1;
  for (let i = 0; i < edgeCount; i += 1) {
    const [parent, child] = edges[i];
    nodes[parent].children.push(nodes[child]);
  }
  const explored = new Set();
  const stack = [nodes[0]];
  let currentNode = 0;

  const output = Array(n);

  while (stack.length) {
    currentNode = stack.pop();
    const {
      children, id, subTreeLabels, label: currentLabel,
    } = currentNode;
    if (explored.has(id)) {
      const numChildren = children.length;
      for (let i = 0; i < numChildren; i += 1) {
        const currentChild = children[i];
        const childLabels = Object.entries(currentChild.subTreeLabels);
        const numLabels = childLabels.length;
        for (let j = 0; j < numLabels; j += 1) {
          const [label, subTreeCount] = childLabels[j];
          currentNode.subTreeLabels[label] = currentNode.subTreeLabels[label]
            ? currentNode.subTreeLabels[label] + subTreeCount
            : subTreeCount;
        }
      }
      output[id] = subTreeLabels[currentLabel];
    } else {
      explored.add(id);
      stack.push(currentNode, ...children);
    }
  }
  return output;
};

const n = 7;
const nodes = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]];
const labels = "abaedcd";

/* 
fails for cases where the child is given before the parent
4

[[0,2],[0,3],[1,2]]

"aeed"

*/

countSubTrees(n, nodes, labels);
