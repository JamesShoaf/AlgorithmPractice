import unittest
from word_search import Solution

class TestWordSearch(unittest.TestCase):
    solver = Solution()
    matrix = [
        ['A','B','C','E'],
        ['S','F','C','S'],
        ['A','D','E','E'],
    ]
    
    def test_empty_matrices(self):
        empty_matrices = [
            [],
            [
                [],
            ],
        ]
        for matrix in empty_matrices:
            self.assertEqual(self.solver.word_search(matrix, 'foo'), False)
            self.assertEqual(self.solver.word_search(matrix, ''), True)
    
    def test_full_matrix(self):
        test_tuples = [
            ('', True),
            ('A', True),
            ('AB', True),
            ('ABC', True),
            ('ABCB', False),
            ('SEE', True),
            ('ABCCED', True),
        ]
        for word, expected in test_tuples:
            self.assertEqual(self.solver.word_search(self.matrix, word), expected)

if __name__ == '__main__':
    unittest.main()