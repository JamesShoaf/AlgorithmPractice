const { RandomizedSet } = require('./randomizedSet');

describe('RandomizedSet', () => {
  let set;
  beforeEach(() => {
    set = new RandomizedSet();
  });

  describe('insert', () => {
    test('it should return true when adding a new value to the set', () => {
      expect(set.insert(3)).toBe(true);
    });

    test('it should return false when adding an existing value to the set', () => {
      set.insert(3);
      expect(set.insert(3)).toBe(false);
    });
  });

  describe('remove', () => {
    test('it should return false when removing a new value from the set', () => {
      expect(set.remove(3)).toBe(false);
    });

    test('it should return true when removing an existing value from the set', () => {
      set.insert(3);
      expect(set.remove(3)).toBe(true);
    });
  });

  describe('getRandom', () => {
    test('it should return a value from the set', () => {
      set.insert(3);
      expect(set.getRandom()).toBe(3);
    });
  });
});
