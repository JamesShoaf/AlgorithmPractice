const { islandCounter } = require('./islandCounter');

describe('islandCounter', () => {
  const islandGrids = [
    [],
    [
      [0],
    ],
    [
      [1],
    ],
    [
      [0, 0],
      [1, 1],
    ],
    [
      [1, 0],
      [0, 1],
    ],
    [
      [1, 0, 1, 0],
      [1, 1, 1, 0],
      [0, 0, 0, 1],
    ],
    [
      [0, 1, 0, 1],
      [1, 0, 1, 0],
      [0, 1, 0, 1],
      [1, 0, 1, 0],
    ],
    [
      [1, 1, 0, 1],
      [1, 0, 0, 0],
      [0, 0, 0, 1],
      [1, 0, 1, 1],
    ],
    [
      [1, 1, 0, 0],
      [0, 0, 1, 1],
      [1, 1, 0, 0],
      [0, 0, 1, 1],
    ],
    [
      ["1","1","1","1","0"],
      ["1","1","0","1","0"],
      ["1","1","0","0","0"],
      ["0","0","0","0","0"],
    ],
    [
      ["1","1","0","0","0"],
      ["1","1","0","0","0"],
      ["0","0","1","0","0"],
      ["0","0","0","1","1"],
    ],
  ];

  test('it should return an integer', () => {
    islandGrids.forEach((grid) => {
      expect(Number.isInteger(islandCounter(grid))).toBe(true);
    });
  });

  test('it should return the right number of islands', () => {
    expect(islandCounter(islandGrids[0])).toBe(0);
    expect(islandCounter(islandGrids[1])).toBe(0);
    expect(islandCounter(islandGrids[2])).toBe(1);
    expect(islandCounter(islandGrids[3])).toBe(1);
    expect(islandCounter(islandGrids[4])).toBe(2);
    expect(islandCounter(islandGrids[5])).toBe(2);
    expect(islandCounter(islandGrids[6])).toBe(8);
    expect(islandCounter(islandGrids[7])).toBe(4);
    expect(islandCounter(islandGrids[8])).toBe(4);
    expect(islandCounter(islandGrids[9])).toBe(1);
    expect(islandCounter(islandGrids[10])).toBe(3);
  });
});
