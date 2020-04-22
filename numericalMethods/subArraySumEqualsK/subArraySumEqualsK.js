const subArraySumEqualsK = (array, k = null) => {
  const sums0Toi = {
    0: [-1], // symbolic 0 runningSum at index -1 representing trivial subArrays [0,i]
  };
  const { length } = array;
  let runningSum = 0;
  let solutions = 0;

  for (let i = 0; i < length; i += 1) {
    const currentInt = array[i];
    runningSum += currentInt;
    if (sums0Toi[runningSum] === undefined) {
      sums0Toi[runningSum] = [];
    }
    sums0Toi[runningSum].push(i);
  }
  if (k === null) { // utility method, return the sums0Toi object if k is omitted
    return sums0Toi;
  }

  const matchableRunningTotals = Object.keys(sums0Toi).map(
    (key) => Number.parseInt(key, 10),
  ).filter((val) => sums0Toi[val - k] !== undefined);

  for (let i = 0; i < matchableRunningTotals.length; i += 1) {
    const currentTotal = matchableRunningTotals[i];
    const rightHandIndices = sums0Toi[currentTotal];
    const leftHandIndices = sums0Toi[currentTotal - k];
    let leftHandIndexIndex = leftHandIndices.length - 1;
    let biggestLeftHandIndex = leftHandIndices[leftHandIndexIndex];
    const smallestLeftHandIndex = leftHandIndices[0];

    for (let j = rightHandIndices.length - 1; j >= 0; j -= 1) {
      const currentRightHandIndex = rightHandIndices[j];
      if (currentRightHandIndex <= smallestLeftHandIndex) {
        break;
      }
      while (
        biggestLeftHandIndex > smallestLeftHandIndex
        && biggestLeftHandIndex >= currentRightHandIndex
      ) {
        leftHandIndexIndex -= 1;
        biggestLeftHandIndex = leftHandIndices[leftHandIndexIndex];
      }
      solutions += leftHandIndexIndex + 1;
    }
  }
  return solutions;
};

const testArray = [1, -1, 0, 1, 1, -1, 0, -1];
subArraySumEqualsK(testArray, -2);

module.exports = {
  subArraySumEqualsK,
};
