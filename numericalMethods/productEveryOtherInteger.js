const productEveryOtherInteger = (array) => {
  if (array.length < 2) {
    return null;
  }
  const output = [];
  let runningProduct = 1;
  for (let i = 0; i < array.length; i += 1) {
    output[i] = runningProduct;
    runningProduct *= array[i];
  }
  runningProduct = 1;
  for (let j = array.length - 1; j >= 0; j -= 1) {
    output[j] *= runningProduct;
    runningProduct *= array[j];
  }
  return output;
};

module.exports = { productEveryOtherInteger };
