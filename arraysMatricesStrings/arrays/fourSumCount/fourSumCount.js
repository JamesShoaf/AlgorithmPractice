// Given four lists A, B, C, D of integer values, compute how many tuples (i, j, k, l) there are
// such that A[i] + B[j] + C[k] + D[l] is zero.

const fourSumCount = (numsA, numsB = [], numsC = [], numsD = []) => {
  let sum = 0;

  // for each pair of lists, map out all potential sums
  const twoSumMap = (numsX, numsY) => {
    const sumMap = {};
    const lengthX = numsX.length;
    const lengthY = numsY.length;
    for (let i = 0; i < lengthX; i += 1) {
      const x = numsX[i];
      for (let j = 0; j < lengthY; j += 1) {
        const xy = x + numsY[j];
        if (sumMap[xy] === undefined) sumMap[xy] = 0;
        sumMap[xy] += 1;
      }
    }
    return sumMap;
  };

  const ab = twoSumMap(numsA, numsB);
  const cd = twoSumMap(numsC, numsD);

  // for each pair of complimentary maps, count the number of combinations that sum to 0
  const twoMapCount = (mapX, mapY) => {
    const xKeys = Object.keys(mapX).map((key) => Number(key));
    const { length } = xKeys;
    for (let i = 0; i < length; i += 1) {
      const key = xKeys[i];
      if (mapY[-key]) sum += mapX[key] * mapY[-key];
    }
  };
  twoMapCount(ab, cd);
  return sum;
};

module.exports = { fourSumCount };
