/*
We write the integers of A and B (in the order they are given) on two separate horizontal lines.

Now, we may draw connecting lines: a straight line connecting two numbers A[i] and B[j] such that:

    A[i] == B[j];
    The line we draw does not intersect any other connecting (non-horizontal) line.

Note that a connecting lines cannot intersect even at the endpoints:
each number can only belong to one connecting line.

Return the maximum number of connecting lines we can draw in this way.
*/

// dp[i][j] = (A[i] === B[j]) ? dp[i + 1][j + 1] + 1 : Math.max(dp[i + 1][j], dp[i][j + 1]);

const maxUncrossedMatches = (arrA, arrB) => {
  const { length: rows } = arrA;
  const { length: cols } = arrB;
  let lastRow = [...Array(cols)].map(() => 0);
  for (let i = rows - 1; i >= 0; i -= 1) {
    const aVal = arrA[i];
    const currentRow = [];
    for (let j = cols - 1; j >= 0; j -= 1) {
      currentRow[j] = (aVal === arrB[j])
        ? (lastRow[j + 1] || 0) + 1
        : Math.max(lastRow[j], (currentRow[j + 1] || 0));
    }
    lastRow = currentRow;
  }
  return lastRow[0];
};

module.exports = { maxUncrossedMatches };
