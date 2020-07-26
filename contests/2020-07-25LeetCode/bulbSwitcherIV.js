/*
There is a room with n bulbs, numbered from 0 to n-1, arranged in a row from left to right.
Initially all the bulbs are turned off.

Your task is to obtain the configuration represented by target where target[i] is '1' if the
i-th bulb is turned on and is '0' if it is turned off.

You have a switch to flip the state of the bulb, a flip operation is defined as follows:

    Choose any bulb (index i) of your current configuration.
    Flip each bulb from index i to n-1.

When any bulb is flipped it means that if it is 0 it changes to 1 and if it is 1 it changes to 0.

Return the minimum number of flips required to form target.
*/

const minFlips = (target) => {
  const { length } = target;
  let toggle = false;
  let count = 0;
  for (let i = 0; i < length; i += 1) {
    if ((!toggle && target[i] === '1') || (toggle && target[i] === '0')) {
      toggle = !toggle;
      count += 1;
    }
  }
  return count;
};

const test = '001011101';
const output = minFlips(test);
console.log(output);