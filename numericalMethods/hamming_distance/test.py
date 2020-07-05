import unittest
from hamming_distance import Solution

class TestHammingDistance(unittest.TestCase):
    solver = Solution()
    def test_invalid_input(self):
        invalid_input = [
            ('a', 0),
            (0, 'a'),
            (2.5, 0),
            (0, 2.5),
            (True, 0),
            (0, True),
        ]
        for (x, y) in invalid_input:
            self.assertEqual(self.solver.hamming_distance(x, y), -1)
    
    
    def test_integers(self):
        test_tuples = [
            ((0, 0), 0),
            ((0, -1), 32),
            ((-1, 0), 32),
            (((2 ** 31) - 1, 0), 31),
        ]
        for ((x, y), expected) in test_tuples:
            self.assertEqual(self.solver.hamming_distance(x, y), expected)

if __name__ == '__main__':
    unittest.main()