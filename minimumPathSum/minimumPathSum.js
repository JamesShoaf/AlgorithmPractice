const minimumPathSum = (grid) => {
  if (!Array.isArray(grid) || grid.length === 0) { return 0; } // validate input

  const lastRowIndex = grid.length - 1;
  const lastColIndex = grid[0].length - 1;

  const map = {};

  const priorityQueue = [
    [0, 0, 0], // distance from origin to node, row, col
  ];
  const insertIntoQueue = (insert) => {
    const insertDistance = insert[0];
    const { length } = priorityQueue;
    for (let i = 0; i < length; i += 1) {
      const currentDist = priorityQueue[i][0];
      if (insertDistance < currentDist) {
        priorityQueue.splice(i, 0, insert);
        return true;
      }
    }
    priorityQueue.push(insert);
    return true;
  };
  let found = false;

  const recursiveTraverse = (distanceToNode, row, col) => {
    const distanceThroughNode = distanceToNode + grid[row][col];
    if (!map[[row, col]] || distanceThroughNode < map[[row, col]]) {
      map[[row, col]] = distanceThroughNode;
    } else {
      return false;
    }
    if (row === lastRowIndex && col === lastColIndex) {
      found = true;
      return true;
    }
    if (row !== lastRowIndex) {
      insertIntoQueue([distanceThroughNode, row + 1, col]);
    }
    if (col !== lastColIndex) {
      insertIntoQueue([distanceThroughNode, row, col + 1]);
    }
    return false;
  };

  while (!found) {
    const currentNode = priorityQueue.shift();
    recursiveTraverse(...currentNode);
  }

  return map[[lastRowIndex, lastColIndex]];
};

// minimumPathSum([
//   [1, 3, 1],
//   [1, 5, 1],
//   [4, 2, 1],
// ]);

minimumPathSum([[1, 2, 5], [3, 2, 1]]);

module.exports = {
  minimumPathSum,
};
