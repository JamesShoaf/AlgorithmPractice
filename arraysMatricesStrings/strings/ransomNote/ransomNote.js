/*
Given an arbitrary ransom note string and another string containing letters from all the magazines,
write a function that will return true if the ransom note can be constructed from the magazines;
otherwise, it will return false.

Each letter in the magazine string can only be used once in your ransom note.
*/

const ransomNote = (note, magazines) => {
  const charMap = {};
  magazines.split('').forEach((char) => {
    if (charMap[char]) {
      charMap[char] += 1;
    } else {
      charMap[char] = 1;
    }
  });
  const noteChars = note.split('');
  for (let i = 0; i < noteChars.length; i += 1) {
    if (charMap[noteChars[i]]) {
      charMap[noteChars[i]] -= 1;
    } else {
      return false;
    }
  }
  return true;
};

module.exports = {
  ransomNote,
};
