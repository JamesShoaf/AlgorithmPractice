const lastStoneWeight = (stones) => {
  let crushed = stones.slice();
  while (crushed.length > 1) {
    const freshlyCrushed = [];
    let largest = 0;
    let second = 0;
    for (let i = 0; i < crushed.length; i += 1) {
      const currentStone = crushed[i];
      if (currentStone >= second) {
        if (second > 0) {
          freshlyCrushed.push(second);
        }
        if (currentStone >= largest) {
          [second, largest] = [largest, currentStone];
        } else {
          second = currentStone;
        }
      } else if (currentStone > 0) {
        freshlyCrushed.push(currentStone);
      }
    }
    const fragment = largest - second;
    if (fragment) {
      freshlyCrushed.push(fragment);
    }
    crushed = freshlyCrushed;
  }
  return (crushed.length === 1 && crushed[0] > 0) ? crushed[0] : 0;
};

module.exports = lastStoneWeight;
