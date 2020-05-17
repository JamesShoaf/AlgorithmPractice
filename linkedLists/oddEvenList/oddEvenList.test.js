const List = require('../singlyLinkedList');
const { oddEvenList } = require('./oddEvenList');

const testLists = [];

const listA = new List();
testLists.push(listA);

const listB = new List();
listB.addToTail(1);
testLists.push(listB);

const listC = new List();
listC.addToTail(1);
listC.addToTail(2);
testLists.push(listC);

const listD = new List();
listD.addToTail(1);
listD.addToTail(2);
listD.addToTail(3);
testLists.push(listD);

const listE = new List();
listE.addToTail(1);
listE.addToTail(2);
listE.addToTail(3);
listE.addToTail(4);
testLists.push(listE);

describe('oddEvenList', () => {
  test('it should move all even nodes after all odd nodes', () => {
    const expected = [
      [],
      [1],
      [1, 2],
      [1, 3, 2],
      [1, 3, 2, 4],
    ];
    testLists.forEach((list, index) => {
      expect(List.print(oddEvenList(list))).toEqual(expected[index]);
    });
  });

  test('it should correctly reassign the head and tail of the list', () => {
    const headExpected = [undefined, 1, 1, 1, 1];
    const tailExpected = [undefined, 1, 2, 2, 4];
    testLists.forEach((list, index) => {
      expect(list.head?.value).toBe(headExpected[index]);
      expect(list.tail?.value).toBe(tailExpected[index]);
    });
  });
  
});
