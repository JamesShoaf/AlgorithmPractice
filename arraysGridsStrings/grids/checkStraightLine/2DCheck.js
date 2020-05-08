const checkStraightLine = (coordinates) => {
  if (coordinates.length <= 2) return true;
  const [[x1, y1], [x2, y2]] = coordinates;
  const { length: coordLength } = coordinates;
  if (x1 === x2) {
    for (let i = 1; i < coordLength; i += 1) {
      const [currentX] = coordinates[i];
      if (currentX !== x1) return false;
    }
  } else {
    const slope = (y2 - y1) / (x2 - x1);
    const intercept = y1 - slope * x1;
    for (let i = 2; i < coordLength; i += 1) {
      const [currentX, currentY] = coordinates[i];
      if (currentY - slope * currentX !== intercept) return false;
    }
  }
  return true;
};

module.exports = {
  checkStraightLine,
};
