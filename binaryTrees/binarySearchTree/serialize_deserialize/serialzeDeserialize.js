// Definition for a binary tree node.
class TreeNode {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

const serialize = (root) => {
  let output = '';
  if (root === null) return output;
  const queue = [root];
  let i = 0;
  const pushStr = (node) => {
    if (node === null) {
      output += 'x';
    } else {
      output += String(node.val);
      queue.push(node.left);
      queue.push(node.right);
    }
    i += 1;
  };
  pushStr(root);
  while (i < queue.length) {
    output += ',';
    pushStr(queue[i]);
  }
  return output;
};

const deserialize = (s) => {
  if (s.length === 0) return null;
  const strings = s.split(',');
  const root = new TreeNode(parseInt(strings[0], 10));
  let i = 1;
  const queue = [root];
  let j = 0;
  while (j < queue.length) {
    const current = queue[j];
    if (i < strings.length && strings[i] !== 'x') {
      current.left = new TreeNode(parseInt(strings[i], 10));
      queue.push(current.left);
    }
    i += 1;
    if (i < strings.length && strings[i] !== 'x') {
      current.right = new TreeNode(parseInt(strings[i], 10));
      queue.push(current.right);
    }
    i += 1;
    j += 1;
  }
  return root;
};

/*
#1 solution for comparison
var serialize = function(root) {
    const str = [];

    postOrderTraversal(root);

    return str.join("#");

    function postOrderTraversal(node) {
        if (node) {
            postOrderTraversal(node.left);
            postOrderTraversal(node.right);
            str.push(node.val);
        }
    }
};

var deserialize = function(data) {
    if (data.length == 0) return null;

    const nums = data.split("#");

    let index = nums.length - 1;

    return postOrderTraversal(Number.MIN_SAFE_INTEGER, Number.MAX_SAFE_INTEGER);

    function postOrderTraversal(lowerBound, upperBound) {
        if (index < 0) return null;
        if (lowerBound > upperBound) return null;

        const curr_val = parseInt(nums[index]);

        if (curr_val < lowerBound || curr_val > upperBound) return null;

        const curr_node = new TreeNode(curr_val);
        index--;

        curr_node.right = postOrderTraversal(curr_val, upperBound);
        curr_node.left = postOrderTraversal(lowerBound, curr_val);

        return curr_node;
    }

};
*/
