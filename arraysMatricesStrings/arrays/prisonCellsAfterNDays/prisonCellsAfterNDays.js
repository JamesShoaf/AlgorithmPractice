const prisonCellsAfterNDays = (cells, N) => {
  const prisonWidth = cells.length;
  const cellMap = {};
  let loopSize = 256;
  let currentPrison = cells.slice();
  const pastPrisons = [];
  for (let i = 0; i < N; i += 1) {
    const cellString = currentPrison.join('');
    if (cellMap[cellString] !== undefined) {
      loopSize = i - cellMap[cellString];
      const loopOffset = i - loopSize;
      return pastPrisons[((N - loopOffset) % loopSize) + loopOffset];
    }
    cellMap[cellString] = i;
    pastPrisons.push(currentPrison);
    const nextPrison = currentPrison.slice();
    for (let j = 0; j < prisonWidth; j += 1) {
      nextPrison[j] = Number(j > 0 && j < prisonWidth - 1
        && currentPrison[j - 1] === currentPrison[j + 1]);
    }
    currentPrison = nextPrison;
  }
  // if N is less than the number of days for the pattern to loop,
  // return the current prison
  return currentPrison;
};

module.exports = {
  prisonCellsAfterNDays,
};
