/*
const mystery = (arr, l, r) => {
  if (r < l) return -1000000000;
  let ans = arr[l];
  for (let i = l + 1; i <= r; i += 1) {
    ans &= arr[i];
  }
  return ans;
}

Winston was given the above mysterious function func. He has an integer array arr and an integer target and he wants to find the values l and r that make the value |func(arr, l, r) - target| minimum possible.

Return the minimum possible value of |func(arr, l, r) - target|.

Notice that func should be called with the values l and r where 0 <= l, r < arr.length.
*/