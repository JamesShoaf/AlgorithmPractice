const spiralOrder = (matrix) => {
  const output = [];
  if (matrix.length === 0) return output;
  let top = 0;
  let bottom = matrix.length - 1;
  let left = 0;
  let right = matrix[0].length - 1;
  while (left <= right && top <= bottom) {
    for (let col = left; col <= right; col += 1) {
      output.push(matrix[top][col]);
    }
    for (let row = top + 1; row <= bottom; row += 1) {
      output.push(matrix[row][right]);
    }
    if (bottom !== top) {
      for (let col = right - 1; col >= left; col -= 1) {
        output.push(matrix[bottom][col]);
      }
    }
    if (right !== left) {
      for (let row = bottom - 1; row > top; row -= 1) {
        output.push(matrix[row][left]);
      }
    }
    top += 1;
    bottom -= 1;
    left += 1;
    right -= 1;
  }
  return output;
};

module.exports = { spiralOrder };
