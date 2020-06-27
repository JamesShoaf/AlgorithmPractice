import unittest
from perfect_squares import Solution

class TestPerfectSquares(unittest.TestCase):
    solver = Solution()
    def test_invalid_input(self):
        invalid_inputs = [
            None,
            -1,
            -5,
            2.0,
            'foo',
        ]
        for invalid in invalid_inputs:
            self.assertEqual(self.solver.numSquares(invalid), -1)
    

    def test_integers(self):
        test_tuples = [
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 1),
            (5, 2),
            (6, 3),
            (7, 4),
            (8, 2),
            (9, 1),
            (10, 2),
            (11, 3),
            (12, 3),
            (13, 2),
            (100, 1),
            (10000, 1),
            (3006, 3),
            (6110, 3),
        ]
        for num, expected in test_tuples:
            self.assertEqual(self.solver.numSquares(num), expected)

if __name__ == '__main__':
    unittest.main()