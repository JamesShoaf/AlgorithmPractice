// const productEveryOtherInteger = (array) => {
//   if (array.length < 2) {
//     return null;
//   }
//   const output = [];
//   for (let i = 0; i < array.length; i += 1) {
//     output.push(1);
//   }
//   for (let i = 0; i < array.length; i += 1) {
//     for (let j = 0; j < array.length; j+= 1) {
//       const multiplyBy = (i === j ? 1 : array[i]);
//       output[j] *= multiplyBy;
//     }
//   }
//   return output;
// }


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

// const productEveryOtherInteger = (array) => {
//   if (array.length < 2) {
//     return null;
//   }
//   const output = [];
//   const zeroes = [];
//   let runningProduct = 1;
//   for (let i = 0; i < array.length; i += 1) {
//     if (array[i] === 0) {
//       zeroes.push(i);
//       continue;
//     }
//     runningProduct *= array[i];
//   }
//   if (zeroes.length > 1) {
//     for (let j = 0; j < array.length; j += 1) {
//       output.push(0);
//     }
//   } else if (zeroes.length === 1) {
//     for (let j = 0; j < array.length; j += 1) {
//       if (j === zeroes[0]) {
//         output.push(runningProduct);
//       } else {
//         output.push(0);
//       }
//     }
//   } else {
//     for (let j = 0; j < array.length; j += 1) {
//       output.push(runningProduct / array[j]);
//     }
//   }
//   return output;
// }