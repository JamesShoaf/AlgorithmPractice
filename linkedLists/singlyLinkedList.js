const Node = require('./singlyLinkedNode');

class SinglyLinkedList {
  constructor() {
    this.head = null;
    this.tail = null;
  }

  addToTail(value) {
    const currentNode = value instanceof Node ? value : new Node(value);
    if (this.head === null) {
      this.head = currentNode;
    }
    if (this.tail) {
      this.tail.next = currentNode;
    }
    this.tail = currentNode;
  }

  removeHead() {
    const { head } = this;
    if (head === null) {
      return head;
    }
    if (this.tail === head) {
      this.tail = null;
    }
    this.head = head.next;
    return head.value;
  }

  checkCycle() {
    let tortoise = this.head;
    let hare = this.head;

    while (hare && hare.next) {
      tortoise = tortoise.next;
      hare = hare.next.next;
      if (tortoise === hare) {
        return true;
      }
    }
    return false;
  }

  startCycle() {
    let tortoise = this.head;
    let hare = this.head;

    while (hare && hare.next) {
      tortoise = tortoise.next;
      hare = hare.next.next;
      if (tortoise === hare) {
        tortoise = this.head;
        while (tortoise !== hare) {
          tortoise = tortoise.next;
          hare = hare.next;
        }
        return hare;
      }
    }
    return null;
  }

  static print(list) {
    const output = [];
    let { head: node } = list;
    while (node) {
      output.push(node.value);
      node = node.next;
    }
    return output;
  }

  static fromArray(arr) {
    const list = new SinglyLinkedList();
    arr.forEach((val) => list.addToTail(val));
    return list;
  }
}

module.exports = SinglyLinkedList;
