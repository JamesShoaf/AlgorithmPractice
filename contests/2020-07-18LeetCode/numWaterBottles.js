/*
Given numBottles full water bottles, you can exchange numExchange empty water bottles for one
full water bottle.

The operation of drinking a full water bottle turns it into an empty bottle.

Return the maximum number of water bottles you can drink.
*/

const numWaterBottles = (numBottles, numExchange) => {
  let fullBottles = numBottles;
  let empties = 0;
  let bottlesDrunk = 0;
  while (fullBottles > 0) {
    bottlesDrunk += fullBottles;
    empties += fullBottles;
    fullBottles = Math.floor(empties / numExchange);
    empties %= numExchange;
  }
  return bottlesDrunk;
};

module.exports = { numWaterBottles };
