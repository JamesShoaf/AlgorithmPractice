const touchToneKnightsMove = (number, length = 9) => {
  // sanitize input
  if (!Number.isInteger(number) || number > 9 || number < 0) return null;
  if (length === 0) return 1;
  // map possible moves from each integer
  const knightMap = {
    1: [6, 8],
    2: [7, 9],
    3: [4, 8],
    4: [3, 9, 0],
    5: [],
    6: [1, 7, 0],
    7: [2, 6],
    8: [1, 3],
    9: [2, 4],
    0: [4, 6],
  };

  // create an array of all possible endings for the starting number
  // (an array of 0s with a 1 at the index matching number)
  let possibleEndings = [...Array(10)].map(() => 0);
  possibleEndings[number] = 1;

  // map current move endings to new move endings the desired number of times
  for (let i = 0; i < length; i += 1) {
    // generate a new array filled with 0s
    const newEndings = [...Array(10)].map(() => 0);
    // for each ending digit
    possibleEndings.forEach((possibilityCount, index) => {
      // look at the possible knight's moves from that digit
      const possibilities = knightMap[index];
      // for possible move
      possibilities.forEach((possibleValue) => {
        // add the number of current moves ending there to the new array
        newEndings[possibleValue] += possibilityCount;
      });
    });
    // update the map with all new move endings
    possibleEndings = newEndings;
  }
  // add up all possible move endings
  return possibleEndings.reduce((acc, val) => acc + val);
};

module.exports = touchToneKnightsMove;
