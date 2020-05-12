class MakeChange {
  constructor(denominations) {
    this.denominations = denominations;
    this.memo = {};
    denominations.forEach((k, index) => {
      this.memo[JSON.stringify([0, index])] = 1;
    });
    const [smallest] = denominations;
    for (let i = 1; i < smallest; i += 1) {
      this.memo[JSON.stringify([i, 0])] = 0;
    }
  }

  makeChange(sum) {
    const {
      memo,
      denominations,
    } = this;
    const recursiveChange = (remaining, index) => {
      const argString = JSON.stringify([remaining, index]);
      if (memo[argString] !== undefined) return memo[argString];
      let possibilities = 0;
      const remainingAfterCurrentCoin = remaining - denominations[index];
      if (remainingAfterCurrentCoin >= 0) {
        possibilities += recursiveChange(remaining - denominations[index], index);
      }
      if (index > 0) {
        possibilities += recursiveChange(remaining, index - 1);
      }
      memo[argString] = possibilities;
      return possibilities;
    };
    return recursiveChange(sum, denominations.length - 1);
  }
}

const makeChange = (sum, denominations) => {
  const waysToMakeChange = [...new Array(sum + 1)].fill(0);
  waysToMakeChange[0] = 1;
  denominations.forEach((coin) => {
    for (let i = coin; i <= sum; i += 1) {
      const remainder = i - coin;
      waysToMakeChange[i] += waysToMakeChange[remainder];
    }
  });
  return waysToMakeChange[sum];
};

module.exports = {
  MakeChange,
  makeChange,
};
