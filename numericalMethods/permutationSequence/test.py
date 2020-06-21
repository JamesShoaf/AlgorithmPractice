import unittest
from permutation_sequence import Solution

class TestPermutationSequence(unittest.TestCase):
    solver = Solution()
    def test_permutations_1(self):
        'it should return the kth permutation of 1 number'
        test_tuples = [
            (1, 1, '1'),
        ]
        for n, k, expected in test_tuples:
            self.assertEqual(self.solver.getPermutation(n, k), expected)

    def test_permutations_2(self):
        'it should return the kth permutation of 2 numbers'
        test_tuples = [
            (2, 1, '12'),
            (2, 2, '21'),
        ]
        for n, k, expected in test_tuples:
            self.assertEqual(self.solver.getPermutation(n, k), expected)
    
    def test_permutations_3(self):
        'it should return the kth permutation of 3 numbers'
        test_tuples = [
            (3, 1, '123'),
            (3, 2, '132'),
            (3, 3, '213'),
            (3, 4, '231'),
            (3, 5, '312'),
            (3, 6, '321'),
        ]
        for n, k, expected in test_tuples:
            self.assertEqual(self.solver.getPermutation(n, k), expected)

if __name__ == '__main__':
    unittest.main()