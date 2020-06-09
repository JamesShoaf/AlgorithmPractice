/*
Given an array nums containing n + 1 integers where each integer is between 1 and n (inclusive),
prove that at least one duplicate number must exist. Assume that there is only one duplicate
number, find the duplicate one.
*/

const findSingleDuplicate = (nums) => {
  // Beacuse each integer is in the range [1, n], each can be used as a pointer to indexes [1, n]
  // Each value at indexes [1, n] will either point to its own index (a loop of size one)
  // or point to another index [1, n], which, if not a duplicate, will point to a different index
  // (forming a loop of size > 1)
  // This makes it a good case for Floyd's tortoise and hare
  // No value points to index 0, so the value at index 0 can be treated as the head of the list
  const head = nums[0];
  let tortoise = head;
  let hare = head;
  do {
    tortoise = nums[tortoise];
    hare = nums[nums[hare]];
  }
  while (tortoise !== hare);
  hare = head;
  while (tortoise !== hare) {
    tortoise = nums[tortoise];
    hare = nums[hare];
  }
  return tortoise;
};

module.exports = { findSingleDuplicate };
