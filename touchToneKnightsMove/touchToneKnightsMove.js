const touchToneKnightsMove = (number, length = 9) => {
  if (typeof number !== 'number'
    || !Number.isInteger(number)
    || number > 9
    || number < 0) {
    return null;
  }
  let sum = 1;
  if (length === 0) {
    return sum;
  }
  const knightMap = {
    1: [6, 8],
    2: [7, 9],
    3: [4, 8],
    4: [3, 9],
    5: [],
    6: [1, 7],
    7: [2, 6],
    8: [1, 3],
    9: [2, 4],
    0: [4, 6],
  };
  return sum;
};

module.exports = touchToneKnightsMove;
