// linked list implementation (O(n^2) time, O(n) space)

// class ListNode {
//   constructor(val = 0, next = null) {
//     this.val = val;
//     this.next = next;
//   }
// }

// const nthUglyNumber = (n) => {
//   if (!Number.isInteger(n) || n < 1) return -1;
//   let nodeCount = 1;
//   let currentNode = new ListNode(1);
//   for (let i = 1; i < n; i += 1, currentNode = currentNode.next) {
//     const { val } = currentNode;
//     let insertionPointer = currentNode;
//     const valsToInsert = [5 * val, 3 * val, 2 * val];
//     while (valsToInsert.length) {
//       const nextVal = valsToInsert.pop();
//       while (insertionPointer && insertionPointer.val < nextVal) {
//         const nextNode = insertionPointer.next;
//         // add a node to the tail of the list only if there are fewer than n nodes total
//         if ((nextNode && nextNode.val > nextVal) || (!nextNode && nodeCount < n)) {
//           insertionPointer.next = new ListNode(nextVal, nextNode);
//           nodeCount += 1;
//         }
//         insertionPointer = insertionPointer.next;
//       }
//     }
//   }
//   return currentNode.val;
// };

// bottom-up memoized implementation (O(n) time, O(n) space)

const nthUglyNumber = (n) => {
  if (!Number.isInteger(n) || n < 1) return -1;

  const uglyNumbers = [1];
  let last = 1;

  let twosIndex = 0;
  let twosVal = 2;

  let threesIndex = 0;
  let threesVal = 3;

  let fivesIndex = 0;
  let fivesVal = 5;

  while (uglyNumbers.length < n) {
    last = Math.min(twosVal, threesVal, fivesVal);
    uglyNumbers.push(last);
    if (last === twosVal) {
      twosIndex += 1;
      twosVal = uglyNumbers[twosIndex] * 2;
    }
    if (last === threesVal) {
      threesIndex += 1;
      threesVal = uglyNumbers[threesIndex] * 3;
    }
    if (last === fivesVal) {
      fivesIndex += 1;
      fivesVal = uglyNumbers[fivesIndex] * 5;
    }
  }

  return last;
};

module.exports = { nthUglyNumber };
