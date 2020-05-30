// search for an element in an array that is otherwise sorted,
// but has been pivoted around one index - eg [3, 4, 5, 1, 2]

const searchRotatedArray = (array, searchElement) => {
  const { length } = array;
  const recursiveSearch = (firstIndex, lastIndex, pivotExcluded = false) => {
    // if the search edges go past each other, the element is not in the array
    if (lastIndex < firstIndex) {
      return -1;
    }
    const middleIndex = Math.floor((lastIndex - firstIndex) / 2) + firstIndex;
    const first = array[firstIndex];
    const last = array[lastIndex];
    const mid = array[middleIndex];
    if (searchElement === first) {
      return firstIndex;
    }
    if (searchElement === last) {
      return lastIndex;
    }
    if (middleIndex === firstIndex) { // skip the rest of the call for 1 and 2 element subsets
      return -1;
    }
    if (searchElement === mid) {
      return middleIndex;
    }

    if (pivotExcluded || first < last) { // the pivot is outside first and last
      return (mid < searchElement)
        ? recursiveSearch(middleIndex + 1, lastIndex - 1, true)
        : recursiveSearch(firstIndex + 1, middleIndex - 1, true);
    } // last < first
    if (first < mid) { // pivot in right half, last < first < mid
      if (first < searchElement && searchElement < mid) {
        // first < searchElement < mid
        return recursiveSearch(firstIndex + 1, middleIndex - 1, true);
      }
      // searchElement < first < mid
      return recursiveSearch(middleIndex + 1, lastIndex - 1);
    } // mid < first, last < first
    if (mid < searchElement && searchElement < last) {
      return recursiveSearch(middleIndex + 1, lastIndex - 1, true);
    }
    return recursiveSearch(firstIndex + 1, middleIndex - 1);
  };
  return recursiveSearch(0, length - 1);
};

const testArray = [5, 1, 2, 3, 4];
searchRotatedArray(testArray, 1);

module.exports = {
  searchRotatedArray,
};
