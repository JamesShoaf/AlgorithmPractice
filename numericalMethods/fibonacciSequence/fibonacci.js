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

module.exports = {
  fibonacci,
};
