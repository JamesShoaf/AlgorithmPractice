const List = require('./singlyLinkedList');
const Node = require('./singlyLinkedNode');

describe('Linked List basics', () => {
  test('it adds a single value to the list', () => {
    const list = new List();
    list.addToTail(1);
    expect(list.head.value).toBe(1);
    expect(list.tail.value).toBe(1);
  });

  test('it adds a second value to the list', () => {
    const list = new List();
    list.addToTail(3);
    list.addToTail(4);
    expect(list.head.value).toBe(3);
    expect(list.head.next.value).toBe(4);
    expect(list.tail.value).toBe(4);
  });

  test('it adds existing nodes to the list', () => {
    const list = new List();
    const node = new Node('a');
    list.addToTail(node);
    expect(list.head.value).toBe('a');
  });

  test('it returns the value of the head node when removed', () => {
    const list = new List();
    list.addToTail(5);
    expect(list.removeHead()).toBe(5);
    expect(list.head).toBe(null);
    expect(list.tail).toBe(null);

    const list2 = new List();
    list2.addToTail(6);
    list2.addToTail(7);
    expect(list2.removeHead()).toBe(6);
    expect(list2.head.value).toBe(7);
    expect(list2.tail.value).toBe(7);
  });

  test('it returns null when removeHead() is called on an empty list', () => {
    const list = new List();
    expect(list.removeHead()).toBe(null);
  });
});

describe('Looping Lists', () => {
  test('checkCycle returns false if the list does not loop', () => {
    const list = new List();
    expect(list.checkCycle()).toBe(false);
  });

  test('checkCycle returns true if the list loops', () => {
    // trivial loop
    const list = new List();
    list.addToTail('o');
    list.tail.next = list.head;
    expect(list.checkCycle()).toBe(true);

    // 3 node loop
    const list2 = new List();
    const loopNode = new Node(0);
    list2.addToTail(1);
    list2.addToTail(2);
    loopNode.next = list2.head;
    list2.addToTail(loopNode);
    expect(list2.checkCycle()).toBe(true);
  });

  test('startCycle returns null if no loop is present', () => {
    const list = new List();
    expect(list.startCycle()).toBe(null);
  });

  test('startCycle returns the first node of the loop if a loop is present', () => {
    const list = new List();
    list.addToTail('a');
    list.addToTail('b');
    list.addToTail('c');
    const d = new Node('d');
    d.next = list.head.next; // b
    list.addToTail(d);
    expect(list.startCycle().value).toBe('b');
  });
});

describe('print', () => {
  test('it should return an array of node values for the list', () => {
    const list = new List();
    expect(List.print(list)).toEqual([]);
    list.addToTail(1);
    expect(List.print(list)).toEqual([1]);
    list.addToTail(2);
    expect(List.print(list)).toEqual([1, 2]);
    list.removeHead();
    expect(List.print(list)).toEqual([2]);
  });
});

describe('fromArray', () => {
  test('it should convert an array of values to a list', () => {
    const testArrays = [
      [],
      [1],
      [1, 2],
      [1, 2, 3, 4, 5, 6, 7, 8],
      ['foo', true, false, Symbol('bar'), {}, []],
    ];
    testArrays.forEach((arr) => {
      expect(List.print(List.fromArray(arr))).toEqual(arr);
    });
  });
});
