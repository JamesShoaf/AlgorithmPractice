class FirstUnique {
  constructor(nums) {
    this.head = null;
    this.seenOnce = new Set();
    this.seenTwice = new Set();
    nums.forEach((num) => {
      this.enqueue(num);
    });
  }

  enqueue(val) {
    const {
      head,
      seenOnce,
      seenTwice,
    } = this;

    if (seenTwice.has(val)) return this;
    if (seenOnce.has(val)) {
      seenOnce.delete(val);
      seenTwice.add(val);
      if (head.val === val) {
        while (seenTwice.has(this.head.val)) {
          const { next } = this.head;
          this.head.next = null;
          this.head = next;
        }
      }
      return this;
    }
    seenOnce.add(val);

    const listNode = {
      val,
      next: null,
    };

    if (head === null) {
      this.head = listNode;
      return this;
    }

    let { next } = head;
    if (next === null) {
      head.next = listNode;
      return this;
    }

    while (next.next !== null) {
      next = next.next;
    }

    next.next = listNode;
    return this;
  }

  showFirstUnique() {
    return this.head.val;
  }
}
