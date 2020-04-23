// Given a range [m, n] where 0 <= m <= n <= 2147483647, return the bitwise
// AND of all numbers in this range, inclusive.

const rangeBitwiseAnd = (m, n) => {
  if (m === n) {
    return m;
  }
  let result = 0;
  const binaryChopper = (subM, subN) => {
    if (subM === 0 || subN === 0) {
      return 0;
    }
    const binaryM = Number(subM).toString(2);
    const binaryN = Number(subN).toString(2);
    if (binaryN.length !== binaryM.length) {
      return 0;
    }
    const powerOf2 = 2 ** (binaryN.length - 1);
    result += powerOf2;
    return binaryChopper(subM - powerOf2, subN - powerOf2);
  };
  binaryChopper(m, n);
  return result;
};

rangeBitwiseAnd(6, 7);

module.exports = {
  rangeBitwiseAnd,
};
