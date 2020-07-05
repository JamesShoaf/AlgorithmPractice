/*
We have a wooden plank of the length n units. Some ants are walking on the plank, each ant moves
with speed 1 unit per second. Some of the ants move to the left, the other move to the right.

When two ants moving in two different directions meet at some point, they change their directions
and continue moving again. Assume changing directions doesn't take any additional time.

When an ant reaches one end of the plank at a time t, it falls out of the plank imediately.

Given an integer n and two integer arrays left and right, the positions of the ants moving to
the left and the right. Return the moment when the last ant(s) fall out of the plank.
*/

const getLastMoment = (n, left, right) => {
  if (!left.length && !right.length) return 0;
  // two rows representing the current wooden plank and the previous wooden plank
  const board = [...Array(2)].map(() => [...Array(n + 1)].map(() => []));
  // leftward ants are represented by a 0
  for (let i = 0; i < left.length; i += 1) {
    board[0][left[i]].push(0);
  }
  // rightward ants are represented by a 1
  for (let i = 0; i < right.length; i += 1) {
    board[0][right[i]].push(1);
  }
  for (let timer = 0; true; timer += 1) {
    let antsSeen = false;
    for (let i = 0; i <= n; i += 1) {
      const currentSegment = board[timer % 2][i];
      if (currentSegment.length) antsSeen = true;
      const collision = Boolean(currentSegment.length > 1);
      while (currentSegment.length) {
        const ant = currentSegment.pop();
        if ((ant && collision) || (!ant && !collision)) {
          if (i > 0) board[(timer + 1) % 2][i - 1].push(0);
        } else if (i < n) board[(timer + 1) % 2][i + 1].push(1);
      }
    }
    if (!antsSeen) return timer - 1;
  }
};

// const n = 4;
// const left = [4, 3];
// const right = [0, 1];
// const output = getLastMoment(n, left, right);
// console.log(output);

module.exports = { getLastMoment };
