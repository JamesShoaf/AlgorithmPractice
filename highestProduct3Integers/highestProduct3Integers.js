// Given an array of three integers, find the highest product you can get from three of the integers

const highestProduct3Integers = (array) => {
  //edge case: if there are only 3 integers, return their product
  if (array.length === 3) {
    return array[0] * array[1] * array[2];
  }
  //initialize 1st, 2nd, and 3rd highest, as well as 1st and 2nd lowest
  let highest = 0;
  let lowest = 0;
  let secondHighest = highest;
  let thirdHighest = highest;
  let secondLowest = lowest;

  // iterate through array
  for (let i = 0; i < array.length; i += 1) {
    // for each integer
    const currentInt = array[i];
    // check if it's higher than the third highest
    if (currentInt >= thirdHighest) {
      // and then if it's higher than the second highest
      if (currentInt >= secondHighest) {
        //and then if it's higher than the highest and update integers accordingly
        if (currentInt >= highest) {
          [thirdHighest, secondHighest, highest] = [secondHighest, highest, currentInt];
        } else {
          [thirdHighest, secondHighest] = [secondHighest, currentInt];
        }
      } else {
        thirdHighest = currentInt;
      }
    }
    // then, if it's negative
    if (currentInt < 0) {
      // check if it's lower than the second lowest
      if (currentInt <= secondLowest) {
        // and then if it's lower than than the lowest and update integers accordingly
        if (currentInt <= lowest) {
          [secondLowest, lowest] = [lowest, currentInt];
        } else {
          secondLowest = currentInt;
        }
      }
    }
  }
  // finally, compare if the two lowest integers have a higher product than the two second highest integers
  // and return that product times the highest positive integer
  const highProduct = secondHighest * thirdHighest;
  const lowProduct = lowest * secondLowest;
  return Math.max(highProduct, lowProduct) * highest;
};

// Given an array of n integers, find the highest product of k of them

// store the highest and lowest integers that could make up the largest product
// high values array length is k
// low values array length is k - k%2

// iterate through the array
  // if k >= 0, push currentInt to high values
    // binary sort high values
    // if high values length > k, pop off the smallest
  // if k < 0, push currentInt to low values
    // binary sort low values
    // if low values length > k - k%2, pop off the largest

// multiply pairs of high values together into a high product array (and keep a reference to the last integer if k is odd)
// multiply pairs of low values together into a low product array
// iterate forward through low products and backwards through high products, comparing their values
// stop when the first high product greater than the low product is found. store index - 1
// multiply together all the low products up to the stored index, all the high products up to k - index, and the odd positive integer out
// return that value
// O(lnk (sort) * n (linear iteration) * k (multiplication)) = O(nklnk) time
// O(k) space