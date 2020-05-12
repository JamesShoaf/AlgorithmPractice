/*
Given a rectangular pizza represented as a rows x cols matrix containing the following characters:
'A' (an apple) and '.' (empty cell) and given the integer k. You have to cut the pizza into k
pieces using k-1 cuts.

For each cut you choose the direction: vertical or horizontal, then you choose a cut position
at the cell boundary and cut the pizza into two pieces. If you cut the pizza vertically, give
the left part of the pizza to a person. If you cut the pizza horizontally, give the upper part
of the pizza to a person. Give the last piece of pizza to the last person.

Return the number of ways of cutting the pizza such that each piece contains at least one apple.
Since the answer can be a huge number, return this modulo 10^9 + 7.
*/

const ways = (pizza, k, topping = 'A') => {
  const memo = {};
  const hasTopping = (slice) => slice.some((row) => Boolean(row.indexOf(topping) > -1));
  const recursiveSlice = (...args) => {
    const argString = JSON.stringify(args);
    if (memo[argString]) return memo[argString];
    const [slice, piecesToCut] = args;
    let output = 0;
    if (piecesToCut === 1) {
      if (hasTopping(slice)) {
        output += 1;
      }
      memo[argString] = output;
      return output;
    }
    let rowFlag = false;
    for (let row = 1; row < slice.length; row += 1) {
      if (!rowFlag) {
        const rowSlice = [slice[row - 1]];
        if (hasTopping(rowSlice)) rowFlag = true;
      }
      if (rowFlag) {
        const remainder = slice.slice(row);
        output += recursiveSlice(remainder, piecesToCut - 1);
      }
    }
    let colFlag = false;
    for (let col = 1; col < slice[0].length; col += 1) {
      if (!colFlag) {
        const colSlice = slice.map((row) => [row[col - 1]]);
        if (hasTopping(colSlice)) colFlag = true;
      }
      if (colFlag) {
        const remainder = slice.map((row) => row.slice(col));
        output += recursiveSlice(remainder, piecesToCut - 1);
      }
    }
    memo[argString] = output;
    return output;
  };
  return recursiveSlice(pizza, k);
};

module.exports = {
  ways,
};
