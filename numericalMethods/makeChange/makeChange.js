// takes an array of integers representing coin values
class MakeChange {
  constructor(denominations) {
    this.denominations = denominations;
    this.memo = {};
    // there is 1 way to make 0 cents with any coin (0 of that coin)
    denominations.forEach((k, index) => {
      this.memo[JSON.stringify([0, index])] = 1;
    });
    const [smallest] = denominations;
    // there are 0 ways to make 1 <= x < smallest cents with any coin
    for (let i = 1; i < smallest; i += 1) {
      this.memo[JSON.stringify([i, 0])] = 0;
    }
  }

  // takes an integer and returns the number of combinations of integers from the initial array
  // that sum to that integer
  makeChange(sum) {
    const {
      memo,
      denominations,
    } = this;
    const recursiveChange = (remaining, index) => {
      // return previous solution for value / coin if there is one
      const argString = JSON.stringify([remaining, index]);
      if (memo[argString] !== undefined) return memo[argString];
      let possibilities = 0;
      // count the possibilities for taking the current coin (current value less coin's value)
      const remainingAfterCurrentCoin = remaining - denominations[index];
      if (remainingAfterCurrentCoin >= 0) {
        possibilities += recursiveChange(remaining - denominations[index], index);
      }
      // count the possibilities for not taking the current coin (taking the previous coin)
      if (index > 0) {
        possibilities += recursiveChange(remaining, index - 1);
      }
      // store the subproblem answer and return it
      memo[argString] = possibilities;
      return possibilities;
    };
    return recursiveChange(sum, denominations.length - 1);
  }
}

const makeChange = (sum, denominations) => {
  // 0 filled array representing ways to make values 0 to sum with 0 coins
  const waysToMakeChange = [...Array(sum + 1)].fill(0);
  // there is 1 way to make 0 cents with 0 coins
  waysToMakeChange[0] = 1;
  // for each coin
  denominations.forEach((coin) => {
    // for each value, add the possibilities for taking the current coin at that value
    for (let i = coin, j = 0; i <= sum; i += 1, j += 1) {
      waysToMakeChange[i] += waysToMakeChange[j];
    }
    // each index now represents the possibilities for taking the current coin and all previous
    // coins at that value
  });
  return waysToMakeChange[sum];
};

module.exports = {
  MakeChange,
  makeChange,
};
