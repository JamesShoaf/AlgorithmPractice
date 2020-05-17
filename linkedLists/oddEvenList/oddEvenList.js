const List = require('../singlyLinkedList');

const oddEvenList = (list) => {
  if (list.head === null) return list;
  const { head } = list;
  const firstEven = head.next;
  if (firstEven === null) return list;
  let odd = head;
  let even = odd.next;
  while (odd.next) {
    odd.next = even.next;
    if (even.next) {
      odd = even.next;
      even.next = odd.next;
      list.tail = (odd.next) ? odd.next : even;
      even = odd.next;
    }
  }
  odd.next = firstEven;
  return list;
};

const listE = new List();
listE.addToTail(1);
listE.addToTail(2);
listE.addToTail(3);

oddEvenList(listE);

module.exports = { oddEvenList };
