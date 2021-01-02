const maxProduct = (nums) => {
  const minus1 = nums.map((num) => num - 1);
  const [a, b] = minus1;
  let maxNum = Math.max(a, b);
  let maxProd = a * b;
  const { length } = minus1;
  for (let i = 2; i < length; i += 1) {
    const currentInt = minus1[i];
    maxProd = Math.max(maxProd, currentInt * maxNum);
    maxNum = Math.max(maxNum, currentInt);
  }
  return maxProd;
};

module.exports = { maxProduct };
