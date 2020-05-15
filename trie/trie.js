class Trie {
  constructor(char) {
    this.val = char;
    this.children = {};
    this.end = false;
  }

  insert(string) {
    let currentNode = this;
    const { length } = string;
    for (let i = 0; i < length; i += 1) {
      const char = string[i];
      const { children } = currentNode;
      if (!children[char]) children[char] = new Trie(char);
      currentNode = children[char];
    }
    currentNode.end = true;
  }

  search(string) {
    let currentNode = this;
    const { length } = string;
    for (let i = 0; i < length; i += 1) {
      const char = string[i];
      const { children } = currentNode;
      if (!children[char]) return false;
      currentNode = children[char];
    }
    return currentNode.end;
  }

  startsWith(string) {
    let currentNode = this;
    const { length } = string;
    for (let i = 0; i < length; i += 1) {
      const char = string[i];
      const { children } = currentNode;
      if (!children[char]) return false;
      currentNode = children[char];
    }
    return true;
  }
}

module.exports = { Trie };
