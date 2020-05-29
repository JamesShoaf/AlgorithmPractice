const circularDependencies = (N, dependencies) => {
  const dependencyMap = [...Array(N)].map(() => []);
  dependencies.forEach(([node, dependency]) => {
    dependencyMap[node].push(dependency);
  });

  const currentPath = new Set();
  const nonCircular = new Set();
  const nodeStack = [];
  // for each node, if it's not already confirmed as nonCircular, trace its path for circles
  for (let i = 0; i < N; i += 1) {
    if (!nonCircular.has(i)) {
      // add the node, and a reference to its first dependency.
      nodeStack.push([i, 0]);
      while (nodeStack.length) {
        // pop the node and index off the stack
        const [currentNode, dependencyIndex] = nodeStack.pop();
        // add the node to the current path
        currentPath.add(currentNode);
        const nodeDependencies = dependencyMap[currentNode];
        const { length } = nodeDependencies;
        // set a flag for if the node has any dependencies that are not confirmed to be noncircular
        let noCircles = true;
        // for each dependency, check if it has been confirmed to be noncircular
        for (let j = dependencyIndex; j < length; j += 1) {
          const currentDependency = nodeDependencies[j];
          // if not
          if (!nonCircular.has(currentDependency)) {
            // if the dependency is circular, return false for the whole graph
            if (currentPath.has(currentDependency)) return false;
            // otherwise, toggle the flag
            noCircles = false;
            // add the current node back to the stack to resume at the next index,
            // and add the current dependency to the stack to loop through again
            nodeStack.push([currentNode, j + 1], [currentDependency, 0]);
            break;
          }
        }
        // if all dependencies have been confirmed to be noncircular
        if (noCircles) {
          // remove the node from the path, and add it to the confirmed noncircular set
          currentPath.delete(currentNode);
          nonCircular.add(currentNode);
        }
      }
    }
  }
  // if all nodes have been confirmed to be noncircular, return true for the whole graph
  return true;
};

module.exports = { circularDependencies };
