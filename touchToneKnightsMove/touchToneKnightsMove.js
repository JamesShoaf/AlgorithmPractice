const touchToneKnightsMove = (number, length = 9) => {
  // sanitize input
  if (typeof number !== 'number'
    || !Number.isInteger(number)
    || number > 9
    || number < 0) {
    return null;
  }
  if (length === 0) {
    return 1;
  }
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
  let possibleEndings = [];
  possibleEndings.length = 10;
  possibleEndings.fill(0, 0, possibleEndings.length);
  possibleEndings[number] = 1;

  // map current move endings to new move endings
  const mapNewEndings = (endings) => {
    // generate a new array filled with 0s
    const newEndings = [];
    newEndings.length = 10;
    newEndings.fill(0, 0, newEndings.length);
    // for each ending digit
    for (let i = 0; i < endings.length; i += 1) {
      // look at the possible knight's moves
      const possibilities = knightMap[i];
      // for each move from integer i
      for (let j = 0; j < possibilities.length; j += 1) {
        // add the number of current moves ending there to the new array
        newEndings[possibilities[j]] += endings[i];
      }
    }
    // return the map of all new move endings
    return newEndings;
  };
  // map possible endings to new endings the desired number of times
  for (let i = 0; i < length; i += 1) {
    possibleEndings = mapNewEndings(possibleEndings);
  }
  // add up all possible move endings
  return possibleEndings.reduce((acc, val) => (acc + val));
};

module.exports = touchToneKnightsMove;
