/* eslint-disable max-classes-per-file */
class QueueNode {
  constructor(val, prev = null, next = null) {
    this.val = val;
    this.prev = prev;
    this.next = next;
  }
}

class FrontMiddleBackQueue {
  constructor() {
    this.head = null;
    this.middle = null;
    this.tail = null;
    this.nodeCount = 0;
  }

  firstNode(val) {
    this.head = new QueueNode(val);
    this.middle = this.head;
    this.tail = this.head;
  }

  pushFront(val) {
    this.nodeCount += 1;
    if (!this.head) {
      this.firstNode(val);
      return;
    }
    this.head.prev = new QueueNode(val, null, this.head);
    this.head = this.head.prev;
    if (this.nodeCount % 2 === 0) {
      this.middle = this.middle.prev;
    }
  }

  pushBack(val) {
    this.nodeCount += 1;
    if (!this.head) {
      this.firstNode(val);
      return;
    }
    this.tail.next = new QueueNode(val, this.tail);
    this.tail = this.tail.next;
    if (this.nodeCount % 2 === 1) {
      this.middle = this.middle.next;
    }
  }

  pushMiddle(val) {
    this.nodeCount += 1;
    if (!this.head) {
      this.firstNode(val);
      return;
    }
    if (this.nodeCount % 2 === 0) {
      const { prev } = this.middle;
      this.middle.prev = new QueueNode(val, prev, this.middle);
      this.middle = this.middle.prev;
    } else {
      const { next } = this.middle;
      this.middle.next = new QueueNode(val, this.middle, next);
      this.middle = this.middle.next;
    }
  }

  emptyQueue() {
    if (this.nodeCount === 0) {
      return -1;
    }
    const { val } = this.head;
    this.head = null;
    this.tail = null;
    this.middle = null;
    this.nodeCount = 0;
    return val;
  }

  popFront() {
    if (this.nodeCount <= 1) {
      return this.emptyQueue();
    }
    const { val } = this.head;
    this.nodeCount -= 1;
    this.head = this.head.next;
    this.head.prev = null;
    if (this.nodeCount % 2 === 1) {
      this.middle = this.middle.next;
    }
    return val;
  }

  popBack() {
    if (this.nodeCount <= 1) {
      return this.emptyQueue();
    }
    const { val } = this.tail;
    this.nodeCount -= 1;
    this.tail = this.tail.prev;
    this.tail.next = null;
    if (this.nodeCount % 2 === 0) {
      this.middle = this.middle.prev;
    }
    return val;
  }

  popMiddle() {
    if (this.nodeCount <= 1) {
      return this.emptyQueue();
    }
    const { val } = this.middle;
    this.nodeCount -= 1;
    const { prev, next } = this.middle;
    if (prev) {
      prev.next = next;
      this.middle = this.nodeCount % 2 === 0 ? prev : next;
    } else {
      this.head = next;
      this.middle = next;
    }
    next.prev = prev;
    return val;
  }
}

const queue = new FrontMiddleBackQueue();
queue.pushFront(1);
queue.pushBack(2);
queue.pushMiddle(3);
queue.pushMiddle(4);
queue.popFront();
queue.popMiddle();
queue.popMiddle();
queue.popBack();
queue.popFront();

// export default FrontMiddleBackQueue;
