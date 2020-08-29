const pancakeSort = (arr) => {
  const { length } = arr;
  const output = [];
  const flip = (i) => {
    if (i > 0) {
      for (let j = 0; j < (i + 1) / 2; j += 1) {
        [arr[j], arr[i - j]] = [arr[i - j], arr[j]];
      }
      output.push(i + 1);
    }
  };

  for (let i = length - 1; i >= 0; i -= 1) {
    if (arr[i] !== i + 1) {
      const j = arr.indexOf(i + 1);
      if (j === -1) { return []; }
      flip(j);
      flip(i);
    }
  }
  return output;
};

module.exports = { pancakeSort };
