/*
Suppose you have a random list of people standing in a queue. Each person is described by a pair
of integers (h, k), where h is the height of the person and k is the number of people in front of
this person who have a height greater than or equal to h. Write an algorithm to reconstruct the
queue.
*/

const reconstructHeightQueue = (people) => {
  const { length: queueLength } = people;
  const output = Array(queueLength);
  // make a set of open indices
  const indices = new Set();
  for (let i = 0; i < queueLength; i += 1) indices.add(i);
  // create a map of positions in line for each height
  const heightMap = {};
  for (let i = 0; i < queueLength; i += 1) {
    const [height, peopleAhead] = people[i];
    if (!heightMap[height]) heightMap[height] = [];
    heightMap[height].push(peopleAhead);
  }
  // sort the heights in ascending order
  const heights = Object.keys(heightMap)
    .map((key) => Number(key))
    .sort((a, b) => a - b);
  const { length: numHeights } = heights;
  // for each height
  for (let i = 0; i < numHeights; i += 1) {
    const currentHeight = heights[i];
    // sort the positions at the current height in ascending order
    const visibleAtHeight = heightMap[currentHeight].sort((a, b) => a - b);
    // and store the maximum number of members of the queue visible
    // for members of the current height
    const maxVisible = visibleAtHeight[visibleAtHeight.length - 1];
    // collect the open indices from the set
    const openIndices = indices.values();
    // and store indices to mark closed after queueing all members of the current height
    const indicesAtCurrentHeight = [];
    // finally, create a set of visible heights for constant time lookup
    const visibleSet = new Set(visibleAtHeight);

    // for 0 to the maximum number of visible people at the current height
    for (let j = 0; j <= maxVisible; j += 1) {
      // grab the next element in the openIndices set
      const currentIndex = openIndices.next().value;
      // if that open index is the nth open index in the set of indexes for the current height
      if (visibleSet.has(j)) {
        // add it to the set to mark closed
        indicesAtCurrentHeight.push(currentIndex);
        // and add a tuple to the location in the output queue
        output[currentIndex] = [currentHeight, j];
      }
    }
    // after all members of the current height have been added to the queue
    // delete each closed index from the open indices set
    while (indicesAtCurrentHeight.length) indices.delete(indicesAtCurrentHeight.pop());
  }
  // after iterating through each height, return the output
  return output;
};

module.exports = { reconstructHeightQueue };
