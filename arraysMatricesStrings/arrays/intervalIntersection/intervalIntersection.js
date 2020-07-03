const intervalIntersection = (intervalA, intervalB) => {
  const { length: lengthA } = intervalA;
  const { length: lengthB } = intervalB;

  let indexA = 0;
  let [subA] = intervalA;
  const nextA = () => {
    indexA += 1;
    subA = intervalA[indexA];
    return indexA === lengthA;
  };

  let indexB = 0;
  let [subB] = intervalB;
  const nextB = () => {
    indexB += 1;
    subB = intervalB[indexB];
    return indexB === lengthB;
  };

  const output = [];
  while (indexA < lengthA && indexB < lengthB) {
    // while A'ss end is less than B's start (or vice versa)
    while (subA[1] < subB[0] || subB[1] < subA[0]) {
      // while A's end is less than B's start, move A to the next interval.
      // return early if out of bounds
      const bStart = subB[0];
      while (subA[1] < bStart) if (nextA()) return output;
      // repeat for B
      const aStart = subA[0];
      while (subB[1] < aStart) if (nextB()) return output;
    }
    // set the current interval start to the max of A and B's start
    const currentStart = Math.max(subA[0], subB[0]);
    const currentEnd = Math.min(subA[1], subB[1]);
    output.push([currentStart, currentEnd]);
    // increment A and/or B if either ended at the current end
    if (subA[1] === currentEnd) if (nextA()) return output;
    if (subB[1] === currentEnd) if (nextB()) return output;
  }
  return output; // return empty intersection if either intA or intB is empty
};

module.exports = { intervalIntersection };
