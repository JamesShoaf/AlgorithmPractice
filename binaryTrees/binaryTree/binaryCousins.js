/*
In a binary tree, the root node is at depth 0, and children of each depth k node are at depth k+1.

Two nodes of a binary tree are cousins if they have the same depth, but have different parents.

We are given the root of a binary tree with unique values, and the values x and y of two different
nodes in the tree.

Return true if and only if the nodes corresponding to the values x and y are cousins.
*/

const binaryCousins = (root, val1, val2) => {
  let q1 = [root];
  while (q1.length) { // for each generation
    const q2 = []; // playpen for children
    let foundFlag = false; // flag for when one value is found
    for (let i = 0; i < q1.length; i += 1) { // for each node in current generation
      let dadFlag = false; // flag for current node if one value is found
      const dad = q1[i];
      if (dad.left !== null) {
        if (dad.left.val === val1 || dad.left.val === val2) {
          if (foundFlag) return true; // if the other value has been found, return true
          foundFlag = true; // otherwise set the found flag
          dadFlag = true; // and set the flag for the current node if the value is found on the left
        } else q2.push(dad.left); // if no value is found, push the child to the next generation
      }
      if (dad.right !== null) {
        if (dad.right.val === val1 || dad.right.val === val2) {
          if (dadFlag) return false; // the other value was a sibling, so return false
          if (foundFlag) return true;
          foundFlag = true;
        } else q2.push(dad.right);
      }
    }
    // if only one of the values is found in the current generation, return false
    if (foundFlag) return false;
    q1 = q2; // next generation replaces current generation
  }
  return false;
};

module.exports = {
  binaryCousins,
};
