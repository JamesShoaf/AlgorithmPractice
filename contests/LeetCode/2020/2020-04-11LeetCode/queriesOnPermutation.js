/*

Given the array queries of positive integers between 1 and m, you have to process all queries[i]
(from i=0 to i=queries.length-1) according to the following rules:

In the beginning, you have the permutation P=[1,2,3,...,m].
For the current i, find the position of queries[i] in the permutation P (indexing from 0)
and then move this at the beginning of the permutation P. Notice that the position of queries[i]
in P is the result for queries[i].
Return an array containing the result for the given queries.

*/

// generate permutation array [1, 2, 3, ... m]

const queriesOnPermutation = (query, m) => {
  const permute = [];
  for (let i = 0; i < m; i += 1) {
    permute.push(i + 1);
  }

  const output = [];
  for (let i = 0; i < query.length; i += 1) {
    const pIndex = permute.indexOf(query[i]);
    output.push(pIndex);
    permute.splice(0, 0, permute.splice(pIndex, 1)[0]);
  }
  return output;
};

module.exports = queriesOnPermutation;
