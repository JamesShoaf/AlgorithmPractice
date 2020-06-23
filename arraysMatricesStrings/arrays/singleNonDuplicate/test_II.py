import unittest
from single_number_II import Solution

class Test_Single_Number_II(unittest.TestCase):
    solver = Solution()
    def test_positive_integers(self):
        test_tuples = [
            ([1, 1, 1, 2, 2, 2, 3, 3, 3, 4], 4),
            ([5, 5, 5, 4, 2, 2, 2, 8, 8, 8], 4),
            ([5, 5, 5, 4], 4),
        ]
        for nums, expected in test_tuples:
            self.assertEqual(self.solver.singleNumber(nums), expected)
    def test_negative_integers(self):
        test_tuples = [
            ([-1, -1, -1, -2, -2, -2, -3, -3, -3, -4], -4),
            ([-5, -5, -5, -4, -2, -2, -2, -8, -8, -8], -4),
            ([-5, -5, -5, -4], -4),
        ]
        for nums, expected in test_tuples:
            self.assertEqual(self.solver.singleNumber(nums), expected)
    def test_mixed_integers(self):
        test_tuples = [
            ([-1, -1, -1, 2, 2, 2, -3, -3, -3, 4], 4),
            ([1, 1, 1, -2, -2, -2, 3, 3, 3, -4], -4),
            ([5, 5, 5, -4, -2, -2, -2, 8, 8, 8], -4),
            ([-5, -5, -5, 4, 2, 2, 2, -8, -8, -8], 4),
            ([-5, -5, -5, 4], 4),
            ([5, 5, 5, -4], -4),
        ]
        for nums, expected in test_tuples:
            self.assertEqual(self.solver.singleNumber(nums), expected)

if __name__ == '__main__':
    unittest.main()