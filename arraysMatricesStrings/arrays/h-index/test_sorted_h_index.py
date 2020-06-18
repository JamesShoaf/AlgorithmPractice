taget = __import__("sorted")
from sorted import h_index
import unittest

class TestHIndex (unittest.TestCase):
    solver = h_index.Solution()
    def test_zero(self):
        test_tuples = [
            ([], 0),
            ([0], 0),
            ([0, 0], 0),
        ]
        for arr, expected in test_tuples:
            self.assertEqual(self.solver.solve(arr), expected)
    
    def test_nonzero(self):
        # solver = Solution()
        test_tuples = [
            ([1], 1),
            ([2], 1),
            ([1, 1], 1),
            ([1, 2], 1),
            ([2, 2], 2),
            ([3, 3], 2),
            ([0, 1, 2, 2], 2),
            ([0, 3, 3, 3], 3),
            ([3, 3, 3, 3], 3),
            ([0, 1, 3, 5, 6], 3),
            ([0, 1, 1, 1, 1], 1),
            ([0, 1, 2, 5, 6], 2),
            ([0, 4, 4, 5, 6], 4),
        ]
        for arr, expected in test_tuples:
            self.assertEqual(self.solver.solve(arr), expected)

if __name__ == '__main__':
    unittest.main()