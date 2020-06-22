import unittest
from dungeon_game import Solution

class TestDungeonGame(unittest.TestCase):
    solver = Solution()
    def test_empty_dungeons(self):
        test_tuples = [
            (
                [],
                1,
            ),
            (
                [[]],
                1,
            ),
        ]
        for dungeon, expected in test_tuples:
            self.assertEqual(self.solver.calculateMinimumHP(dungeon), expected)
    def test_square_dungeons(self):
        test_tuples = [
            (
                [[-2]],
                3,
            ),
            (
                [
                    [-2, -2],
                    [-3, 10],
                ],
                5,
            ),
            (
                [
                    [-2, -2, -6],
                    [-3, 10, -1],
                    [-3, -1, -1],
                ],
                5,
            ),
            (
                [
                    [-2, -3, -1],
                    [-2, 10, -1],
                    [-3, -1, -1],
                ],
                5,
            ),
            (
                [
                    [1,-3,3],
                    [0,-2,0],
                    [-3,-3,-3]
                ],
                3,
            ),
        ]
        for dungeon, expected in test_tuples:
            self.assertEqual(self.solver.calculateMinimumHP(dungeon), expected)
    def test_tall_dungeons(self):
        test_tuples = [
            (
                [
                    [-1],
                    [-3],
                ],
                5,
            ),
            (
                [
                    [-1, -2],
                    [-3, 2],
                    [-1, -5],
                ],
                7,
            ),
            (
                [
                    [-1, -2, -2],
                    [-3, 2, -2],
                    [-1, -1, 3],
                    [-1, -1, -5],
                ],
                5,
            ),
        ]
        for dungeon, expected in test_tuples:
            self.assertEqual(self.solver.calculateMinimumHP(dungeon), expected)
    def test_wide_dungeons(self):
        test_tuples = [
            (
                [
                    [-1, -2],
                ],
                4,
            ),
            (
                [
                    [-1, -2, -3],
                    [-1, -1, -1],
                ],
                5,
            ),
            (
                [
                    [-1, 0, -3, -1],
                    [-1, -1, -2, -1],
                    [-1, -1, -1, -1],
                ],
                6,
            ),
        ]
        for dungeon, expected in test_tuples:
            self.assertEqual(self.solver.calculateMinimumHP(dungeon), expected)
if __name__ == '__main__':
    unittest.main()