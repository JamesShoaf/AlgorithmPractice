/*
Given a string S and a string T, find the minimum window in S which will contain all the characters
in T in complexity O(n).
*/

const minimumWindowSubstring = (s, t) => {
  // create a map of all characters in t
  const { length: sLength } = s;
  const { length: tLength } = t;
  const counts = {};
  // count all characters in t
  for (let i = 0; i < tLength; i += 1) counts[t[i]] = counts[t[i]] + 1 || 1;
  // create an array of characters from the keys in counts
  const chars = Object.keys(counts);
  // store index 0 and length of chars for iteration in the next step
  const { length: charsLength } = chars;
  let matchedIndex = 0;

  // Phase 1: find the first window which contains all letters in the map
  let start = 0;
  let end;
  for (let i = 0; i < sLength; i += 1) {
    const char = s[i];
    // for each character in s, if it is contained within counts, decrement counts
    if (counts[char] !== undefined) {
      counts[char] -= 1;
      // if counts = 0, mark the character as matched
      if (counts[char] <= 0) {
        // check each character to confirm it has been matched within the window
        for (; matchedIndex < charsLength; matchedIndex += 1) {
          // break early if there is an unmatched character to prevent repeated checks
          if (counts[chars[matchedIndex]] > 0) break;
        }
        // set the ending index to the current index and end the current loop
        if (matchedIndex === charsLength) {
          end = i;
          break;
        }
      }
    }
  }
  // if the end of the end index is not defined, t is not a substring of s
  if (end === undefined) return '';
  // otherwise, store start and end as the current best indexes, and continue
  let bestStart = start;
  let bestEnd = end;
  // end of phase 1

  // alternate phases 2 and 3 to the end of s
  const unmatchedStack = [];
  while (end < sLength && end - start >= tLength - 1) {
    // Phase 2: shrink the window until it is no longer valid
    // iterate the start pointer forward, until removing the character just prior to start
    do {
      start += 1;
      const char = s[start - 1];
      if (counts[char] !== undefined) {
        counts[char] += 1;
        // add that character to the unmatched stack
        unmatchedStack.push(char);
        // store start - 1 and end as the current best start and end
        bestStart = start - 1;
        bestEnd = end;
        break;
      }
    } while (start < end);

    // Phase 3: shift the window forward to find a new valid window
    // move start and end forward, removing start - 1, and adding end
    do {
      start += 1;
      end += 1;
      const startChar = s[start - 1];
      const endChar = s[end];
      // each time a character in chars is removed
      if (counts[startChar] !== undefined) {
        counts[startChar] += 1;
        // if counts is > 0, add it to a stack of unmatched characters
        if (counts[startChar] > 0) unmatchedStack.push(startChar);
      }
      // each time a character is added
      if (counts[endChar] !== undefined) {
        counts[endChar] -= 1;
        // if the character is fully matched
        if (counts[endChar] === 0) {
          // pop characters off the stack and see if they are matched
          while (unmatchedStack.length) {
            const unmatchedChar = unmatchedStack.pop();
            // if an unmatched character is found, push it back on the stack and stop popping
            if (counts[unmatchedChar] > 0) {
              unmatchedStack.push(unmatchedChar);
              break;
            }
          }
          // if the stack is empty, return to phase 2
          if (unmatchedStack.length === 0) break;
        }
      }
    } while (end < sLength);
  }

  return s.slice(bestStart, bestEnd + 1);
};

const stringA = 'ab';
const stringB = 'b';
minimumWindowSubstring(stringA, stringB);

module.exports = { minimumWindowSubstring };
