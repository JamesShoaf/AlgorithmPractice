import unittest
from unique_binary_search_trees import Solution

class TestNumTrees(unittest.TestCase):
    solver = Solution()
    def test_non_positive_non_integer(self):
        test_tuples = [
            (-1, 1),
            (0, 1),
            ('a', 0),
            (2.0, 0),
        ]
        for n, expected in test_tuples:
            self.assertEqual(self.solver.numTrees(n), expected)


    def test_positive(self):
        test_tuples = [
            (1, 1),
            (2, 2),
            (3, 5),
            (4, 14),
            (5, 42),
        ]
        for n, expected in test_tuples:
            self.assertEqual(self.solver.numTrees(n), expected)

if __name__ == '__main__':
    unittest.main()