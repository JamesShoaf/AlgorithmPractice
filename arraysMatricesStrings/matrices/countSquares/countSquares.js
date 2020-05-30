/*
Given a m * n matrix of ones and zeros, return how many square submatrices have all ones.
*/

// O(n) space, O(n^3) time
// with massive amortization improvements that bring it closer to O(sqrt(n)) space O(n^2) time

const countSquares = (matrix) => {
  if (matrix.length === 0 || matrix[0].length === 0) return 0;
  const rows = matrix.length;
  const cols = matrix[0].length;
  let previousRowMap = [];
  let numberOfSquares = 0;
  // fill out a map of the largest square possible with [row, col] as the superior corner
  for (let row = 0; row < rows; row += 1) {
    let currentRowMap = Array(cols);
    // for each square, if the square is 1, add it to the map
      for (let col = 0; col < cols; col += 1) {
        // skip the loops if the current square is not 1.
        if (matrix[row][col] !== 1) currentRowMap[col] = 0;
        else {
          // check the map at [row - 1, col] and [row, col - 1] and set the value to whichever is higher - 1
          let squares = Math.max(Math.max(currentRowMap[col - 1] ?? 0, previousRowMap[col] ?? 0) - 1, 1);
          // for each larger possible square size, check the last row and last column for validity
          nextSquare: for (let nextLarger = squares + 1;; nextLarger += 1) {
            // if dimensions are impossible, break early
            if (row + nextLarger > rows || col + nextLarger > cols) break;
            // check right column, (break if a 0 is found or if out of bounds)
            for (let subRow = row; subRow < row + nextLarger; subRow += 1) {
              if (matrix[subRow][col + nextLarger - 1] !== 1) break nextSquare;
            }
            // check bottom row,
            // subCol -1 since we've already checked the bottomr right corner in the previous step
            for (let subCol = col; subCol < col + nextLarger - 1; subCol += 1) { // -2
              if (matrix[row + nextLarger - 1][subCol] !== 1) break nextSquare;
            }
            // if the loop hasn't been broken, the larger square is valid.
            // Update squares and move to next larger size
            squares = nextLarger;
          }
          // add the final squares value to the current row map
          currentRowMap[col] = squares;
          // and update the final total
          numberOfSquares += squares;
        }
      }
      // after finishing each row, we no longer need to store the last row, so overwrite it.
      previousRowMap = currentRowMap;
    }
    // after all squares have been mapped, return the sum.
    return numberOfSquares;
  };

module.exports = { countSquares };