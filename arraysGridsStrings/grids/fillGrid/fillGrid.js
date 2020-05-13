const fillGrid = (...args) => {
  // assign values to rows, cols, and fill for either options object
  // or positional parameters
  const [rows, cols, fill = 0] = (typeof args[0] === 'object')
    ? (() => {
      const [{ rows: rowVal, cols: colVal, fill: fillVal = 0 }] = args;
      return [rowVal, colVal, fillVal];
    })()
    : args;
  // throw an error if row or column count is not a positive integer
  if (
    !Number.isInteger(rows)
    || !Number.isInteger(cols)
    || rows < 1
    || cols < 1
  ) {
    throw new Error('invalid array dimensions');
  }
  return [...Array(rows)]
    .map(() => [...Array(cols)]
      .map(() => fill));
};

module.exports = {
  fillGrid,
};
