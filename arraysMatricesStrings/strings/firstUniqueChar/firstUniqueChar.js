const firstUniqueChar = (string) => {
  const charMap = {};
  for (let i = 0; i < string.length; i += 1) {
    const char = string[i];
    if (charMap[char] === undefined) {
      charMap[char] = i;
    } else {
      charMap[char] = null;
    }
  }
  const chars = new Set(string).entries();
  for (const char of chars) {
    const currentChar = charMap[char[0]];
    if (currentChar !== null) {
      return currentChar;
    }
  }
  return -1;
};

module.exports = {
  firstUniqueChar,
};
