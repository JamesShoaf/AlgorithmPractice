const maximalSquare = (grid) => {
  const numberOfRows = grid.length;
  if (numberOfRows === 0) {
    return 0;
  }
  const numberOfCols = grid[0].length;
  if (numberOfCols === 0) {
    return 0;
  }
  let longestDiag = 0;
  const squareChecker = (squareRow, squareCol) => {
    if (!grid[squareRow + longestDiag]) {
      return false;
    }
    if (grid[squareRow + longestDiag][squareCol + longestDiag] !== 1) {
      return false;
    }
    for (let i = squareRow; i <= squareRow + longestDiag; i += 1) {
      for (let j = squareCol; j <= squareCol + longestDiag; j += 1) {
        if (grid[i][j] !== 1) {
          return false;
        }
      }
    }
    longestDiag += 1;
    return squareChecker(squareRow, squareCol);
  };
  for (let row = 0; row < numberOfRows; row += 1) {
    if (numberOfRows - row <= longestDiag) {
      break;
    }
    for (let col = 0; col < numberOfCols; col += 1) {
      if (numberOfCols - col <= longestDiag) {
        break;
      }
      if (grid[row][col] === 1) {
        squareChecker(row, col);
      }
    }
  }
  return longestDiag ** 2;
};

const testGrid = [
  [1, 0, 1, 0, 0],
  [1, 0, 1, 1, 1],
  [1, 1, 1, 1, 1],
  [1, 0, 0, 1, 0],
];

maximalSquare(testGrid);

module.exports = {
  maximalSquare,
};
