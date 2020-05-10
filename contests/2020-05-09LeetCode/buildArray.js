/*

Given an array target and an integer n. In each iteration, you will read a
number from  list = {1,2,3..., n}.

Build the target array using the following operations:

Push: Read a new element from the beginning list, and push it in the array.
Pop: delete the last element of the array.
If the target array is already built, stop reading more elements.
You are guaranteed that the target array is strictly increasing, only containing
numbers between 1 to n inclusive.

Return the operations to build the target array.

You are guaranteed that the answer is unique.

*/

const buildArray = (target /* , n */) => {
  const output = [];
  let index = 0;
  const { length } = target;
  for (let i = 1; index < length; i += 1) {
    const currentTarget = target[index];
    output.push('Push');
    if (currentTarget !== i) output.push('Pop');
    else index += 1;
  }
  return output;
};

buildArray([1, 3]);

module.exports = {
  buildArray,
};
