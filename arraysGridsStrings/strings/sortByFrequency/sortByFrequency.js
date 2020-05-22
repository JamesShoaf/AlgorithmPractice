const sortByFrequency = (string) => {
  const charMap = {};
  const frequencySets = [new Set()];

  let { length } = string;
  for (let i = 0; i < length; i += 1) {
    const char = string[i];
    const newCount = charMap[char] + 1 || 1;
    charMap[char] = newCount;
    frequencySets[newCount - 1].delete(char);
    if (frequencySets[newCount] === undefined) frequencySets[newCount] = new Set(char);
    else frequencySets[newCount].add(char);
  }
  let output = '';
  length = frequencySets.length;
  for (let i = length - 1; i > 0; i -= 1) {
    for (const char of frequencySets[i]) output += char.repeat(i);
  }
  return output;
};

module.exports = { sortByFrequency };
