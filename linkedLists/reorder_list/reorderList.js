const reorderList = (head) => {
  // count nodes
  let count = 0;
  let temp = head;
  while (temp) {
    count += 1;
    temp = temp.next;
  }

  // split list at midpoint
  temp = head;
  for (let i = 0; i < (count - 2) / 2; i += 1) { temp = temp.next; }
  let head2 = temp ? temp.next : null;
  if (temp) { temp.next = null; }

  // reverse the list from mid forward
  temp = head2;
  let prev = null;
  while (temp) {
    const { next } = temp;
    temp.next = prev;
    prev = temp;
    temp = next;
  }
  head2 = prev;

  // rejoin the two lists
  temp = head;
  while (head2) {
    const next1 = temp.next;
    const next2 = head2.next;
    temp.next = head2;
    head2.next = next1;
    temp = next1;
    head2 = next2;
  }
  return head;
};

module.exports = { reorderList };
