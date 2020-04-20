
const islandCounter = (grid) => {
  const gridHeight = grid.length;
  if (gridHeight === 0) {
    return 0;
  }
  const gridWidth = grid[0].length;
  let scouts = 0;
  const map = {};

  const releaseScout = (row, col) => {
    if (grid[row][col] == 0) {
      return false;
    }
    map[[row, col]] = scouts;
    if (row + 1 < gridHeight && !map[[row + 1, col]]) {
      releaseScout(row + 1, col);
    }
    if (row - 1 >= 0 && !map[[row - 1, col]]) {
      releaseScout(row - 1, col);
    }
    if (col + 1 < gridWidth && !map[[row, col + 1]]) {
      releaseScout(row, col + 1);
    }
    if (col - 1 >= 0 && !map[[row, col - 1]]) {
      releaseScout(row, col - 1);
    }
    return true;
  };

  for (let i = 0; i < gridHeight; i += 1) {
    for (let j = 0; j < gridWidth; j += 1) {
      if (grid[i][j] == 1 && !map[[i, j]]) {
        scouts += 1;
        releaseScout(i, j);
      }
    }
  }
  return scouts;
};

module.exports = { islandCounter };
