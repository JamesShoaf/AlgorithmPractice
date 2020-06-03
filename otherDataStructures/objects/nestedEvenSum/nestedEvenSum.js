const nestedEvenSum = (obj) => {
  let sum = 0;
  if (obj % 2 === 0) sum += obj;
  if (typeof obj === 'object') {
    const entries = Object.entries(obj);
    const { length } = entries;
    for (let i = 0; i < length; i += 1) {
      sum += nestedEvenSum(entries[i][1]);
    }
  }
  return sum;
};

module.exports = { nestedEvenSum };
