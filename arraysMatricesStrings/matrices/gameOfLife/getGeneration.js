const getGeneration = (cells, generations) => {
  // make a copy of cells as universe
  const universe = [];
  cells.forEach((row) => universe.push(row.slice()));
  const numRows = universe.length;
  const numCols = universe[0].length;
  for (let i = 0; i < generations; i += 1) {
    // initialize nextUniverse to 0
    const nextUniverse = [];
    for (let row = 0; row < numRows; row += 1) {
      nextUniverse[row] = [];
      for (let col = 0; col < numCols; col += 1) {
        nextUniverse[row][col] = 0;
      }
    }
    // count neighbors for each cell in next universe
    universe.forEach((row, rowIndex) => row.forEach((col, colIndex) => {
      if (col === 0) return;
      if (rowIndex !== 0) {
        if (colIndex !== 0) {
          nextUniverse[rowIndex - 1][colIndex - 1] += 1;
        }
        nextUniverse[rowIndex - 1][colIndex] += 1;
        if (colIndex !== numCols - 1) {
          nextUniverse[rowIndex - 1][colIndex + 1] += 1;
        }
      }
      if (colIndex !== 0) {
        nextUniverse[rowIndex][colIndex - 1] += 1;
      }
      if (colIndex !== numCols - 1) {
        nextUniverse[rowIndex][colIndex + 1] += 1;
      }
      if (rowIndex !== numRows - 1) {
        if (colIndex !== 0) {
          nextUniverse[rowIndex + 1][colIndex - 1] += 1;
        }
        nextUniverse[rowIndex + 1][colIndex] += 1;
        if (colIndex !== numCols - 1) {
          nextUniverse[rowIndex + 1][colIndex + 1] += 1;
        }
      }
    }));
    // update living and dead cells in universe based on neighbor counts
    universe.forEach((row, rowIndex) => row.forEach((col, colIndex) => {
      if (col === 1) {
        const neighbors = nextUniverse[rowIndex][colIndex];
        if (neighbors > 3 || neighbors < 2) {
          universe[rowIndex][colIndex] = 0;
        }
      } else if (nextUniverse[rowIndex][colIndex] === 3) {
        universe[rowIndex][colIndex] = 1;
      }
    }));
  }
  return universe;
};

module.exports = {
  getGeneration,
};
