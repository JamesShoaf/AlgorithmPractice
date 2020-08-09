/* 
Given two positive integers n and k, the binary string  Sn is formed as follows:

    S1 = "0"
    Si = Si-1 + "1" + reverse(invert(Si-1)) for i > 1

Where + denotes the concatenation operation, reverse(x) returns the reversed string x, and invert(x) inverts all the bits in x (0 changes to 1 and 1 changes to 0).

For example, the first 4 strings in the above sequence are:

    S1 = "0"
    S2 = "011"
    S3 = "0111001"
    S4 = "011100110110001"

Return the kth bit in Sn. It is guaranteed that k is valid for the given n.
*/

const findKthBit = (n, k) => {
  let s = "0";
  const reverse = {
    "0": "1",
    "1": "0",
  };
  let { length } = s;
  while (length < k) {
    let t = "1";
    for (let j = length - 1; j >= 0; j -= 1) {
      t += reverse[s[j]];
    }
    s += t;
    length = s.length;
  }
  return s[k - 1];
};
