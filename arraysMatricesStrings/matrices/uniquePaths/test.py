import unittest
from unique_paths import Solution

class TestUniquePaths(unittest.TestCase):
    solver = Solution()

    def test_invalid_input(self):
        test_tuples = [
            ((0, 1), None),
            ((1, 0), None),
            ((-1, 1), None),
            ((1, -1), None),
            ((2.0, 1), None),
            ((1, 2.0), None),
            (('a', 1), None),
            ((1, 'a'), None),
        ]
        for (m, n), expected in test_tuples:
            self.assertEqual(self.solver.uniquePaths(m, n), expected)
    
    
    def test_integer_input(self):
        test_tuples = [
            ((1, 1), 1),
            ((1, 100), 1),
            ((100, 1), 1),
            ((100, 1), 1),
            ((5, 2), 5),
            ((5, 4), 35),
            ((5, 7), 210),
        ]
        for (m, n), expected in test_tuples:
            self.assertEqual(self.solver.uniquePaths(m, n), expected)

if __name__ == '__main__':
    unittest.main()