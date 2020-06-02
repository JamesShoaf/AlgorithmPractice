const flatten = (array) => ((Array.isArray(array))
  ? array.reduce((acc, val) => acc.concat(flatten(val)), [])
  : array);

module.exports = { flatten };
