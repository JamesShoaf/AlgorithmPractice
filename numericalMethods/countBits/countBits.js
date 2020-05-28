/*
Given a non negative integer number num. For every numbers i in the range 0 ≤ i ≤ num calculate the
number of 1's in their binary representation and return them as an array.
*/

const countBits = (num) => {
  // find the next lower power of two from the number
  const powerOfTwo = Math.floor(Math.log2(num));
  const bitPattern = [
    [0],
    [1],
  ];
  // procedurally generate the next row from the previous one
  // ie 1 => 1, 2 => 1, 2, 2, 3 => ...
  for (let i = 2; i <= powerOfTwo + 1; i += 1) {
    const nextPattern = bitPattern[i - 1];
    bitPattern[i] = nextPattern.concat(nextPattern.map((val) => val + 1));
  }

  return [].concat(...bitPattern).slice(0, num + 1);
};

module.exports = { countBits };
