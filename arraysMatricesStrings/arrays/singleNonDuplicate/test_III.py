import unittest
from single_number_III import Solution

class TestSingleNumberIII(unittest.TestCase):
    solver = Solution()
    def test_positive(self):
        test_tuples = [
            ([1, 1, 2, 2, 15, 15, 16, 16, 3, 4], [3, 4]),
            ([1, 1, 2, 2, 15, 15, 16, 16, -3, -4], [-3, -4]),
            ([1, 1, 2, 2, 15, 15, 16, 16, -3, 4], [-3, 4]),
        ]
        for nums, expected in test_tuples:
            output = self.solver.singleNumberIII(nums)
            for exp in expected:
                self.assertIn(exp, output)
    
    def test_negative(self):
        test_tuples = [
            ([-1, -1, -2, -2, -15, -15, -16, -16, 3, 4], [3, 4]),
            ([-1, -1, -2, -2, -15, -15, -16, -16, -3, -4], [-3, -4]),
            ([-1, -1, -2, -2, -15, -15, -16, -16, -3, 4], [-3, 4]),
        ]
        for nums, expected in test_tuples:
            output = self.solver.singleNumberIII(nums)
            for exp in expected:
                self.assertIn(exp, output)
    
    def test_mixed(self):
        test_tuples = [
            ([-1, -1, 2, 2, 15, 15, -16, -16, 3, 4], [3, 4]),
            ([-1, -1, 2, 2, 15, 15, -16, -16, -3, -4], [-3, -4]),
            ([-1, -1, 2, 2, 15, 15, -16, -16, -3, 4], [-3, 4]),
            ([-1, -1, 2, 2, 15, 15, -16, -16, 0, 100], [0, 100]),
            ([-1, -1, 2, 2, 15, 15, -16, -16, 0, -100], [0, -100]),
        ]
        for nums, expected in test_tuples:
            output = self.solver.singleNumberIII(nums)
            for exp in expected:
                self.assertIn(exp, output)

if __name__ == '__main__':
    unittest.main()