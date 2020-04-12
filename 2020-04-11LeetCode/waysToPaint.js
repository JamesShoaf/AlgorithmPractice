/*

You have a grid of size n x 3 and you want to paint each cell of the grid with exactly
one of the three colours:Red, Yellow or Green while making sure that no two adjacent
cells have the same colour (i.e no two cells that share vertical or horizontal sides
have the same colour).

You are given n the number of rows of the grid.

Return the number of ways you can paint this grid. As the answer may grow large,
the answer must be computed modulo 10^9 + 7.

*/

const waysToPaint = (n) => {
  let solutions = 0;
  const grid = [];
  for (let i = 0; i < 3 * n; i += 1) {
    grid[i] = 0;
  }
  // 0 = unpainted, 1 = Red, 2 = Yellow, 3 = Green
  // each row of the grid is laid out linearly

  const neighborChecker = (index, color) => {
    if (index > 2) { // check above if index is below top row
      if (grid[index - 3] === color) {
        return false;
      }
    }
    if (grid.length - index > 2) { // check below if index is above bottom row
      if (grid[index + 3] === color) {
        return false;
      }
    }
    if (index % 3 !== 0) { // check left if index is not left justified
      if (grid[index - 1] === color) {
        return false;
      }
    }
    if (index % 3 !== 2) { // check right if index is not right justified
      if (grid[index + 1] === color) {
        return false;
      }
    }
    return true;
  };

  const recursivePainter = (index) => {
    for (let i = 1; i < 4; i += 1) {
      if (neighborChecker(index, i)) {
        grid[index] = i;
        if (index === grid.length - 1) {
          solutions += 1;
        } else {
          recursivePainter(index + 1);
        }
        grid[index] = 0;
      }
    }
  };
  recursivePainter(0);
  return solutions % (10 ** 9 + 7);
};
