/* A delivery company wants to build a new service centre in a new city. The company knows the
positions of all the customers in this city on a 2D-Map and wants to build the new centre in a
position such that the sum of the euclidean distances to all customers is minimum.

Given an array positions where positions[i] = [xi, yi] is the position of the ith customer on
the map, return the minimum sum of the euclidean distances to all customers. */

const getMinDistSum = (positions) => {
  const numPositions = positions.length;
  if (numPositions < 2) return 0;
  if (numPositions === 2) {
    return Math.sqrt((positions[0][0] - positions[1][0]) ** 2
    + (positions[0][0] - positions[1][0]) ** 2);
  }
  if (numPositions === 3) {
    // the radius of the circle containing the three points is not the closest
    // there point closest to all three vertices is the fermat point
    // *to do*
    // // find reciprocal slope for each pair
    // const [a, b, c] = positions;
    // const riseAB = b[1] - a[1];
    // const runAB = b[0] - a[0];
    // const slopeAB = riseAB / runAB;
    // const riseBC = c[1] - b[1];
    // const runBC = c[0] - b[0];
    // const slopeBC = riseBC / runBC;
    // const riseAC = c[1] - a[1];
    // const runAC = c[0] - a[0];
    // const slopeAC = riseAC / runAC;

    // // if two slopes are NaN or if any two slopes are equal
    // if (!(
    //   Number(Number.isNaN(slopeAB))
    //   + Number((Number.isNaN(slopeBC))
    //   + Number(Number.isNaN(slopeAC))) > 1
    // ) && !(slopeAB === slopeBC || slopeBC === slopeAC || slopeAB === slopeAC)) {
    //   // find the coefficients for the question of the circle Ax^2 + Ay^2 + Bx + Cy + D = 0
    //   const A = a[0] * (b[1] - c[1]) - a[1] * (b[0] - c[0]) + b[0] * c[1] - c[0] * b[1];
    //   const B = (a[0] ** 2 + a[1] ** 2) * (c[1] - b[1])
    //     + (b[0] ** 2 + b[1] ** 2) * (a[1] - c[1])
    //     + (c[0] ** 2 + c[1] ** 2) * (b[1] - a[1]);
    //   const C = (a[0] ** 2 + a[1] ** 2) * (b[0] - c[0])
    //     + (b[0] ** 2 + b[1] ** 2) * (c[0] - a[0])
    //     + (c[0] ** 2 + c[1] ** 2) * (a[0] - b[0]);
    //   const D = (a[0] ** 2 + a[1] ** 2) * (c[0] * b[1] - b[0] * c[1])
    //     + (b[0] ** 2 + b[1] ** 2) * (a[0] * c[1] - c[0] * a[1])
    //     + (c[0] ** 2 + c[1] ** 2) * (b[0] * a[1] - a[0] * b[1]);
    //   return Math.sqrt((B ** 2 + C ** 2 - 4 * A * D) / (4 * A ** 2));
    // }
  }
  let minX = Number.POSITIVE_INFINITY;
  let minY = Number.POSITIVE_INFINITY;
  let maxX = Number.NEGATIVE_INFINITY;
  let maxY = Number.NEGATIVE_INFINITY;
  for (let i = 0; i < numPositions; i += 1) {
    const [x, y] = positions[i];
    minX = Math.min(minX, x);
    minY = Math.min(minY, y);
    maxX = Math.max(maxX, x);
    maxY = Math.max(maxY, y);
  }
  const MARGIN_OF_ERROR = 10 ** -6;
  let minDistance = Number.POSITIVE_INFINITY;
  while (maxX - minX > MARGIN_OF_ERROR || maxY - minY > MARGIN_OF_ERROR) {
    const midX = (minX + maxX) / 2;
    const midY = (minY + maxY) / 2;
    let midXmaxY = 0;
    let midXminY = 0;
    let maxXmidY = 0;
    let minXmidY = 0;
    for (let i = 0; i < numPositions; i += 1) {
      const [x, y] = positions[i];
      midXmaxY += Math.sqrt((x - midX) ** 2 + (y - maxY) ** 2);
      midXminY += Math.sqrt((x - midX) ** 2 + (y - minY) ** 2);
      maxXmidY += Math.sqrt((x - maxX) ** 2 + (y - midY) ** 2);
      minXmidY += Math.sqrt((x - minX) ** 2 + (y - midY) ** 2);
    }
    const maxYIsCloser = midXmaxY < midXminY;
    const maxXIsCloser = maxXmidY < minXmidY;
    minY = Number(maxYIsCloser) * midY + Number(!maxYIsCloser) * minY;
    maxY = Number(maxYIsCloser) * maxY + Number(!maxYIsCloser) * midY;
    minX = Number(maxXIsCloser) * midX + Number(!maxXIsCloser) * minX;
    maxX = Number(maxXIsCloser) * maxX + Number(!maxXIsCloser) * midX;
    minDistance = Math.min(minDistance, midXmaxY);
  }
  return minDistance;
};

module.exports = { getMinDistSum };
