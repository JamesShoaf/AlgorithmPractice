/* Given a binary string s (a string consisting only of '0' and '1's).

Return the number of substrings with all characters 1's.

Since the answer may be too large, return it modulo 10^9 + 7. */

const numSub = (s) => {
  const { length } = s;
  let oneCount = 0;
  let count = 0;
  for (let i = 0; i < length; i += 1) {
    oneCount = Number(s[i]) * (oneCount + 1);
    count += oneCount;
  }
  return count % (10 ** 9 + 7);
};

module.exports = { numSub };
