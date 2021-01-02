// Given a string s and an integer array indices of the same length.

// The string s will be shuffled such that the character at the ith position moves to indices[i] in
// the shuffled string.

// Return the shuffled string.

const restoreString = (s, indices) => {
  const shuffled = Array(s.length);
  indices.forEach((newIndex, oldIndex) => {
    shuffled[newIndex] = s[oldIndex];
  });
  return shuffled.join('');
};

const s = 'aaiougrt';
const indices = [4,0,2,6,7,3,1,5];
const output = restoreString(s, indices);
