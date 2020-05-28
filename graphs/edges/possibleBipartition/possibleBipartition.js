/*
Given a set of N people (numbered 1, 2, ..., N), we would like to split everyone into two groups
of any size.

Each person may have a crush on some other people, and they should not go into the same group.

Formally, if crushes[i] = [a, b], it means it is not allowed to put the people numbered a and b
into the same group.

Return true if and only if it is possible to split everyone into two groups in this way.
*/

const possibleBipartition = (N, crushes) => {
  // create a set for each person's crushes and admirers
  const persons = [...Array(N)].map(() => new Set());
  const { length } = crushes;
  // for each crush, add both the admirer and crush to each other's set, whether or not the
  // feeling is mutual
  for (let i = 0; i < length; i += 1) {
    const [admirer, crush] = crushes[i];
    persons[admirer - 1].add(crush);
    persons[crush - 1].add(admirer);
  }
  // for each person, add their crushes/admirers to one set, and them to another.
  // traverse all crushes in a chain before returning to sequential order
  const queue = [];
  const visited = new Set();
  const setA = new Set();
  const setB = new Set();
  const partitionPersons = (person) => {
    // skip previously validated nodes
    if (visited.has(person)) return true;
    const crushesSet = persons[person - 1];
    // check if the node is part of setB
    if (setB.has(person)) {
      for (const admirer of crushesSet) {
        if (setB.has(admirer)) return false;
        setA.add(admirer);
        queue.push(admirer);
      }
    }
    if (!setB.has(person)) {
      if (!setA.has(person)) setA.add(person);
      // add all crushes to setB
      for (const admirer of crushesSet) {
        if (setA.has(admirer)) return false;
        setB.add(admirer);
        queue.push(admirer);
      }
    }
    visited.add(person);
    return true;
  };
  for (let i = 0; i < N; i += 1) {
    // add the current node to the queue
    queue.push(i + 1);
    // remove nodes from the queue until the queue is empty.
    while (queue.length) {
      if (!partitionPersons(queue.pop())) return false;
    }
  }
  // if no conflicts admirers have crushes on each other, bipartition is possible.
  return true;
};

module.exports = { possibleBipartition };
