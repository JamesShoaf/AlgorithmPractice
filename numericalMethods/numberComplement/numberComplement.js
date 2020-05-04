const numberComplement = (int) => {
  if (int === 0) return 1;
  const significantBit = Math.floor(Math.log(int) / Math.log(2)) + 1;
  return 2 ** significantBit - int - 1;
};

module.exports = {
  numberComplement,
};
