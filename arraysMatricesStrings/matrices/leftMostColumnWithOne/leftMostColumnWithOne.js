// Given a row-sorted binary matrix binaryMatrix, return leftmost column index(0-indexed)
// with at least a 1 in it. If such index doesn't exist, return -1.

// You can't access the Binary Matrix directly.  You may only access the matrix using a
// BinaryMatrix interface:

// BinaryMatrix.get(x, y) returns the element of the matrix at index (x, y) (0-indexed).
// BinaryMatrix.dimensions() returns a list of 2 elements [n, m], which means the matrix
// is n * m.

const leftMostColumnWithOne = (binaryMatrix) => {
  const [rows, cols] = binaryMatrix.dimensions();
  // initialize leftmost as undefined since no 1s have been found
  let leftMostOneIndex;
  let currentRow = rows - 1;
  let currentCol = cols - 1;

  while (currentCol >= 0 && currentRow >= 0) {
    const currentVal = binaryMatrix.get(currentRow, currentCol);
    if (currentVal === 1) {
      leftMostOneIndex = currentCol;
      currentRow -= 1;
    } else {
      currentCol -= 1;
    }
  }

  return (leftMostOneIndex === undefined)
    ? -1
    : leftMostOneIndex;
};

module.exports = {
  leftMostColumnWithOne,
};
