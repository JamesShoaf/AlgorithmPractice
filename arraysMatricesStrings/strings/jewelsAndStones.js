/*
You're given strings J representing the types of stones that are jewels, and S representing the
stones you have.  Each character in S is a type of stone you have.  You want to know how many of
the stones you have are also jewels.

The letters in J are guaranteed distinct, and all characters in J and S are letters. Letters are
case sensitive, so "a" is considered a different type of stone from "A".
*/

const numJewelsInStones = (J, S) => {
  const jewels = new Set(J);
  return S.split('')
    .map((stone) => jewels.has(stone))
    .reduce((acc, bool) => ((bool) ? acc + 1 : acc), 0);
};

module.exports = {
  numJewelsInStones,
};
