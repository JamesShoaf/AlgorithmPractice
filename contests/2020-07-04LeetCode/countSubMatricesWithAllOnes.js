/* Given a rows * columns matrix mat of ones and zeros, return how many submatrices have all
ones. */

const numSubMat = (mat) => {
  // count the longest row from each top left corner
  // check that the next row has at least that width
  // if not, subtract 1 from the width and check if the next row down has at least that width
  // repeat for all squares
  const rowCount = mat.length;
  const colCount = mat[0].length;
  let rectangles = 0;
  const countRectangles = (rowIndex, colIndex) => {
    let maxWidth = 0;
    for (let col = colIndex; col < colCount; col += 1) {
      if (mat[rowIndex][col] === 1) maxWidth += 1;
      else break;
    }
    for (let row = rowIndex + 1; maxWidth && row < rowCount; row += 1) {
      rectangles += maxWidth;
      for (let col = colIndex; col <= colIndex + maxWidth - 1; col += 1) {
        if (mat[row][col] !== 1) {
          maxWidth = col - colIndex - 1;
        }
      }
    }
    rectangles += maxWidth;
  };
  for (let row = 0; row < rowCount; row += 1) {
    for (let col = 0; col < colCount; col += 1) {
      countRectangles(row, col);
    }
  }
  return rectangles;
};

module.exports = { numSubMat };
