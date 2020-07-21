const removeElements = (head, val) => {
  let current = head;
  while (current && current.val === val) current = current.next;
  if (current) {
    let prev = current;
    while (prev) {
      let { next } = prev;
      while (next && next.val === val) next = next.next;
      prev.next = next;
      prev = next;
    }
  }
  return current;
};

module.exports = { removeElements };
