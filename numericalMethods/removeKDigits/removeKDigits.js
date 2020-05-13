const removeKDigits = (num, k) => {
  if (num === '') return '0';
  // return the number if k is 0
  if (k === 0) return num;
  const { length } = num;
  // return 0 if k removes all digits in the number
  if (k >= length) return '0';
  // remove numbers from the start that are followed by 0s (since leading 0s are ignored)
  for (let i = 1; i <= k; i += 1) {
    if (num[i] === '0') {
      for (let j = i + 1; j <= length; j += 1) {
        if (num[j] !== '0') return (removeKDigits(num.slice(j), k - i));
      }
      // if the rest of the number is 0s, return 0
      return '0';
    }
  }

  let slice = num;
  for (let i = k; i > 0; i -= 1) {
    const { length: sliceLength } = slice;
    let curr = parseInt(slice[0], 10);
    for (let j = 0; j < sliceLength; j += 1) {
      const right = parseInt(slice[j + 1], 10);
      if (curr > right) {
        slice = slice.slice(0, j) + slice.slice(j + 1);
        break;
      }
      curr = right;
    }
    if (slice.length === sliceLength) slice = slice.slice(0, sliceLength - 1);
  }

  return slice;
};

module.exports = {
  removeKDigits,
};
