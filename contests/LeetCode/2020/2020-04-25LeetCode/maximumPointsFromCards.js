// There are several cards arranged in a row, and each card has an associated number of points
// The points are given in the integer array cardPoints.

// In one step, you can take one card from the beginning or from the end of the row. You have
// to take exactly k cards.

// Your score is the sum of the points of the cards you have taken.

// Given the integer array cardPoints and the integer k, return the maximum score you can obtain.

const maximumPointsFromCards = (array, cards) => {
  const leftHand = [0];
  const rightHand = [0];
  let runningTotal = 0;
  for (let i = 0; i < cards; i += 1) {
    runningTotal += array[i];
    leftHand.push(runningTotal);
  }
  const { length } = array;
  if (cards === length) {
    return runningTotal;
  }
  runningTotal = 0;
  for (let i = length - 1; i >= length - cards; i -= 1) {
    runningTotal += array[i];
    rightHand.push(runningTotal);
  }
  runningTotal = 0;
  for (let i = 0; i < cards + 1; i += 1) {
    runningTotal = Math.max(leftHand[i] + rightHand[cards - i], runningTotal);
  }
  return runningTotal;
};

module.exports = {
  maximumPointsFromCards,
};
