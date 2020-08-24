const { Trie } = require('../trie');

/*
class Trie {
  constructor(char) {
    this.children = {};
    this.end = false;
  }
  ...
}
*/

class StreamChecker {
  constructor(words) {
    const { length } = words;
    const trie = new Trie();
    for (let i = 0; i < length; i += 1) { trie.insert(words[i]); }
    this.trie = trie;
    this.stream = [trie];
  }

  query(char) {
    const { stream, trie } = this;
    const { length } = stream;
    const nextStream = [trie];
    let found = false;
    for (let i = 0; i < length; i += 1) {
      const next = stream[i].children[char];
      if (next !== undefined) {
        found = found || next.end;
        nextStream.push(next);
      }
    }
    this.stream = nextStream;
    return found;
  }
}

module.exports = { StreamChecker };
