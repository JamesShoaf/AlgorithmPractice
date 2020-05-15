const { Trie } = require('./trie');

describe('Trie', () => {
  let trieA;

  beforeEach(() => {
    const trie = new Trie();
    trie.insert('apple');
    trieA = trie;
  });

  describe('constructor', () => {
    test('it should generate Tries with the right starting values', () => {
      const [trie, trieB] = [new Trie(), new Trie('b')];
      expect(trie.val).toBe(undefined);
      expect(trieB.val).toBe('b');
      expect(Object.keys(trie.children).length).toBe(0);
      expect(Object.keys(trieB.children).length).toBe(0);
      expect(trie.end).toBe(false);
      expect(trieB.end).toBe(false);
    });
  });

  describe('insert', () => {
    test('it should generate subtries for new branches', () => {
      expect(trieA.children.b).toBe(undefined);
      trieA.insert('b');
      expect(trieA.children.b instanceof Trie).toBe(true);
    });
    test('it should generate new subtries on existing branches', () => {
      expect(trieA.children.a.children.b).toBe(undefined);
      trieA.insert('ab');
      expect(trieA.children.a.children.b instanceof Trie).toBe(true);
    });
  });

  describe('search', () => {
    test('it should return false for words that have not been added to the trie', () => {
      expect(trieA.search('pear')).toBe(false);
      expect(trieA.search('app')).toBe(false);
    });

    test('it should return true for words that have been added to the trie', () => {
      expect(trieA.search('apple')).toBe(true);
      trieA.insert('app');
      expect(trieA.search('app')).toBe(true);
    });
  });

  describe('startsWith', () => {
    test('it should return false for prefixes that have not been added to the trie', () => {
      expect(trieA.startsWith('pear')).toBe(false);
    });

    test('it should return true for words that have been added to the trie', () => {
      expect(trieA.startsWith('apple')).toBe(true);
    });

    test('it should return true for prefixes of words that have been added to the trie', () => {
      expect(trieA.startsWith('app')).toBe(true);
      trieA.insert('pearls');
      expect(trieA.startsWith('pear')).toBe(true);
    });
  });
});
