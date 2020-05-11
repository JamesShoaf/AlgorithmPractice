/*
An image is represented by a 2-D array of integers, each integer representing the pixel value
of the image (from 0 to 65535).

Given a coordinate (sr, sc) representing the starting pixel (row and column) of the flood fill,
and a pixel value newColor, "flood fill" the image.

To perform a "flood fill", consider the starting pixel, plus any pixels connected 4-directionally
to the starting pixel of the same color as the starting pixel, plus any pixels connected
4-directionally to those pixels (also with the same color as the starting pixel), and so on.
Replace the color of all of the aforementioned pixels with the newColor.

At the end, return the modified image.
*/

const floodFill = (image, sr, sc, newColor) => {
  const oldColor = image[sr][sc];
  if (oldColor === newColor) return image;
  const queue = [[sr, sc]];
  const queueDirections = (row, col) => {
    const fourDirections = [
      [row, col + 1],
      [row, col - 1],
      [row + 1, col],
      [row - 1, col],
    ];
    fourDirections.forEach(([rw, cl]) => {
      if (image[rw] !== undefined) {
        if (image[rw][cl] === oldColor) {
          queue.push([rw, cl]);
        }
      }
    });
  };
  for (let i = 0; i < queue.length; i += 1) {
    const [row, col] = queue[i];
    image[row][col] = newColor;
    queueDirections(row, col);
  }
  return image;
};

module.exports = {
  floodFill,
};
