const cakeThief = (maxWeight, cakeTypes) => {
  const loot = [...Array(maxWeight + 1)].fill(0);
  for (let i = 0; i < cakeTypes.length; i += 1) {
    const { weight, value } = cakeTypes[i];
    if (weight === 0) {
      if (value > 0) return Number.POSITIVE_INFINITY;
    } else {
      for (let j = weight; j <= maxWeight; j += 1) {
        const lowerWeight = j - weight;
        loot[j] = Math.max(loot[j], loot[lowerWeight] + value);
      }
    }
  }
  return loot[maxWeight];
};

const zeroOneCakesack = (maxWeight, cakeTypes) => {
  const loot = [...Array(maxWeight + 1)].fill(0);
  for (let i = 0; i < cakeTypes.length; i += 1) {
    const { weight, value } = cakeTypes[i];
    const lastOptimal = loot.slice();
    for (let j = weight; j <= maxWeight; j += 1) {
      const lowerWeight = j - weight;
      loot[j] = Math.max(loot[j], lastOptimal[lowerWeight] + value);
    }
  }
  return loot[maxWeight];
};

module.exports = {
  cakeThief,
  zeroOneCakesack,
};
