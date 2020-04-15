const stringShift = (string, shift) => {
  // handle edge case of empty string;
  if (string === '') {
    return string;
  }
  let result = string.slice();
  const { length } = string;

  // reduce the array of bidirectional instructions to a single instruction
  const shiftVal = shift.reduce((acc, tuple) => (
    tuple[0] === 1 // if the first index of a shift direction is 1, the shift is positive
      ? acc + tuple[1] // so increment the shift
      : acc - tuple[1] // otherwise decrement it
  // then simplify the shift further by taking skipping full cycles of steps
  ), 0) % result.length; // 'banana' shifted to the right 6 is 'banana'

  if (shiftVal > 0) { // right shift
    // result = (result.slice(result.length - shiftVal) + result).slice(result.length);
    result = (string + string);
    result = result.slice(length - shiftVal, 2 * length - shiftVal);
  }

  if (shiftVal < 0) {
    const absShift = Math.abs(shiftVal);
    result = (string + string).slice(absShift, length + absShift);
  }
  return result;
};

module.exports = {
  stringShift,
};
