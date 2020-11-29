/*
You are given two linked lists: list1 and list2 of sizes n and m respectively.

Remove list1's nodes from the ath node to the bth node, and put list2 in their place.

The blue edges and nodes in the following figure incidate the result:

Constraints:

    3 <= list1.length <= 104
    1 <= a <= b < list1.length - 1
    1 <= list2.length <= 104

*/

const mergeInBetween = (list1, a, b, list2) => {
  let preA = list1;
  for (let i = 0; i < a - 1; i += 1) {
    preA = preA.next;
  }
  let postB = preA;
  for (let i = 0; i < b - a + 2; i += 1) {
    postB = postB.next;
  }
  preA.next = list2;
  while (preA.next) {
    preA = preA.next;
  }
  preA.next = postB;
  return list1;
};

export default mergeInBetween;
