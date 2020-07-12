const reverseBits = (n) => {
  let output = 0;
  for (let i = 0; i < 32; i += 1) {
    const bit = (n & (1 << i)) >>> i;
    output += bit * (2 ** (31 - i));
  }
  return output;
};

module.exports = { reverseBits };
