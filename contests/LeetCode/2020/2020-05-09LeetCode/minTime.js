const minTime = (n, edges, hasApple) => {
  const nodes = {};
  let path = -2;
  const convertEdgeToNode = ([parent, child]) => {
    if (nodes[parent]) return nodes[parent].children.push(child);
    nodes[parent] = { children: [child] };
    return nodes[parent];
  };
  const applePath = (node) => {
    if (nodes[node]) {
      const { children } = nodes[node];
      if (children.filter(applePath).length > 0) {
        path += 2;
        return true;
      }
    }
    if (hasApple[node]) {
      path += 2;
      return true;
    }
    return false;
  };
  edges.forEach(convertEdgeToNode);
  applePath(0);
  return Math.max(0, path);
};

module.exports = {
  minTime,
};
