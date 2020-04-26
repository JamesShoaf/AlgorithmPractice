/*
Given an integer array nums and an integer k, return the maximum sum of a non-empty subset of that
array such that for every two consecutive integers in the subset, nums[i] and nums[j], where i < j,
the condition j - i <= k is satisfied.

A subset of an array is obtained by deleting some number of elements (can be zero) from the array,
leaving the remaining elements in their original order.

*/

/*
I - array, k
O - int
C - j - i <= k
E - all negatives
*/

// find the largest possible first element
// if all negative, return the least negative
