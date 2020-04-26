/*
Given a list of lists of integers, nums, return all elements of nums in diagonal order
as shown in the below examples.

Input: nums = [[1,2,3],[4,5,6],[7,8,9]]
Output: [1,4,2,7,5,3,8,6,9]

Input: nums = [[1,2,3,4,5],[6,7],[8],[9,10,11],[12,13,14,15,16]]
Output: [1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16]
*/

// iterate from the last row up, pushing each element to a new array in an object
// with keys for each diagonal

const diagonalTraversal = (nums) => {
  const diagonalMap = {};
  for (let row = nums.length - 1; row >= 0; row -= 1) {
    const currentRow = nums[row];
    for (let col = currentRow.length - 1; col >= 0; col -= 1) {
      const currentDiag = row + col;
      if (diagonalMap[currentDiag] === undefined) {
        diagonalMap[currentDiag] = [nums[row][col]];
      } else {
        diagonalMap[currentDiag].push(nums[row][col]);
      }
    }
  }
  let output = [];
  for (let i = 0; ; i += 1) {
    if (diagonalMap[`${i}`] === undefined) {
      break;
    }
    output = output.concat(diagonalMap[`${i}`]);
  }
  return output;
};

module.exports = {
  diagonalTraversal,
};
