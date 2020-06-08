/*
Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate
(i, ai). n vertical lines are drawn such that the two endpoints of line i is at (i, ai) and
(i, 0). Find two lines, which together with x-axis forms a container, such that the
container contains the most water.
*/

const maxAreaBetweenIndexes = (heights) => {
  let left = 0;
  let right = heights.length - 1;
  let dist;
  let maxArea = 0;
  while (left < right) {
    const [leftHeight, rightHeight] = [heights[left], heights[right]];
    let unchanged = true;
    // calculate the distance between left and right
    dist = right - left;
    // determine if left or right has the lower height and update the maxArea to dist * the lower
    if (leftHeight <= rightHeight) {
      maxArea = Math.max(maxArea, leftHeight * dist);
      // scan the lower pointer inward to find a height >= the distance between left and right
      for (let i = left + 1; i < right; i += 1) {
        if (heights[i] >= leftHeight) {
          // if a higher value is found, set the lower pointer to that
          left = i;
          unchanged = false;
          break;
        }
      }
    }
    // if right < left, move right inward instead
    if (leftHeight > rightHeight) {
      maxArea = Math.max(maxArea, rightHeight * dist);
      for (let i = right - 1; i > left; i -= 1) {
        if (heights[i] >= rightHeight) {
          right = i;
          unchanged = false;
          break;
        }
      }
    }
    // if no higher value is found when moving inward, return maxArea
    if (unchanged) break;
  }
  return maxArea;
};

module.exports = { maxAreaBetweenIndexes };
