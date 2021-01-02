/*
Given a rectangular cake with height h and width w, and two arrays of integers horizontalCuts and
verticalCuts where horizontalCuts[i] is the distance from the top of the rectangular cake to the
ith horizontal cut and similarly, verticalCuts[j] is the distance from the left of the rectangular
cake to the jth vertical cut.

Return the maximum area of a piece of cake after you cut at each horizontal and vertical position
provided in the arrays horizontalCuts and verticalCuts. Since the answer can be a huge number,
return this modulo 10^9 + 7.
*/

const maxArea = (h, w, horizontalCuts, verticalCuts) => {
  const sortHorizontal = horizontalCuts.sort((a, b) => a - b);
  const sortVertical = verticalCuts.sort((a, b) => a - b);
  const { length: hznCuts } = horizontalCuts;
  const { length: vrtCuts } = verticalCuts;
  let maxHeight = Math.max(h - sortHorizontal[hznCuts - 1], sortHorizontal[0]);
  for (let i = 1; i < hznCuts; i += 1) {
    maxHeight = Math.max(maxHeight, sortHorizontal[i] - sortHorizontal[i - 1]);
  }
  let maxWidth = Math.max(w - sortVertical[vrtCuts - 1], sortVertical[0]);
  for (let i = 1; i < vrtCuts; i += 1) {
    maxWidth = Math.max(maxWidth, sortVertical[i] - sortVertical[i - 1]);
  }
  return (maxHeight * maxWidth) % (10 ** 9 + 7);
};

module.exports = { maxArea };
