/*

Given an array of integers arr.

We want to select three indices i, j and k where (0 <= i < j <= k < arr.length).

Let's define a and b as follows:

a = arr[i] ^ arr[i + 1] ^ ... ^ arr[j - 1]
b = arr[j] ^ arr[j + 1] ^ ... ^ arr[k]
Note that ^ denotes the bitwise-xor operation.

Return the number of triplets (i, j and k) Where a == b.

*/

const countTriplets = (arr) => {
  let reduceLeft = arr.reduce((acc, val) => (val ^ acc), 0);
  let output = 0;
  for (let i = 0; i < arr.length - 1; reduceLeft ^= arr[i], i += 1) {
    let reduceRight = reduceLeft;
    for (let k = arr.length - 1; k > i; reduceRight ^= arr[k], k -= 1) {
      if (reduceRight === 0) output += k - i;
    }
  }
  return output;
};

module.exports = {
  countTriplets,
};
