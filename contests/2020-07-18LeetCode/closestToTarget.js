/*
const mystery = (arr, l, r) => {
  if (r < l) return -1000000000;
  let ans = arr[l];
  for (let i = l + 1; i <= r; i += 1) {
    ans &= arr[i];
  }
  return ans;
}

Winston was given the above mysterious function func. He has an integer array arr and an integer
target and he wants to find the values l and r that make the value |func(arr, l, r) - target|
minimum possible.

Return the minimum possible value of |func(arr, l, r) - target|.

Notice that func should be called with the values l and r where 0 <= l, r < arr.length.
*/

const closestToTarget = (arr, target) => {
  let runningTotal = arr[0];
  let closestResult = Math.abs(runningTotal - target);
  const { length } = arr;
  for (let i = 1; i < length; i += 1) {
    // the key here is that the & operator can only subtract from positive integers,
    // so there's no point in &-ing totals that are already below the target
    // if all numbers are below the target, the runningTotal simply runs through each integer
    // otherwise, it ands sequential integers until it goes below the target
    runningTotal = runningTotal < target
      ? arr[i]
      : runningTotal & arr[i];
    closestResult = Math.min(
      closestResult,
      Math.abs(runningTotal - target),
      // in some cases, the running total might dip further below the target by &-ing the
      // next number than the difference between that number and the target.
      Math.abs(target - arr[i]),
    );
  }
  return closestResult;
};

module.exports = { closestToTarget };
