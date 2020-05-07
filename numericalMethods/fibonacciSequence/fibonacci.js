const fibonacci = (int) => {
  if (int === 0) return 0;
  let current = 1;
  let last = 0;
  for (let i = 1; i < int; i += 1) {
    const sum = last + current;
    last = current;
    current = sum;
  }
  return current;
};

class Fib {
  constructor() {
    this.memo = {
      0: 0,
      1: 1,
      2: 1,
    };
  }

  fibonacci(int) {
    const {
      memo,
    } = this;
    if (memo[int] !== undefined) return memo[int];
    const k = (int % 2)
      ? Math.floor((int + 1) / 2)
      : int / 2;
    memo[int] = (int % 2)
      ? (this.fibonacci(k) ** 2 + this.fibonacci(k - 1) ** 2)
      : (2 * this.fibonacci(k - 1) + this.fibonacci(k)) * this.fibonacci(k);
    return memo[int];
  }
}

const fib = new Fib();
fib.fibonacci(50);

module.exports = {
  fibonacci,
  Fib,
};
