from . import longest_duplicate_substring
import unittest

class TestLongestDuplicateSubstring(unittest.TestCase):
    solver = longest_duplicate_substring.Solution()
    def test_empty(self):
        test_tuples = [
            ('', ''),
            ('abcd', ''),
        ]
        for string, expected in test_tuples:
            self.assertEqual(self.solver.longestDupSubstring(string), expected)
    
    def test_non_overlapping(self):
        test_tuples = [
            ('abcda', 'a'),
            ('abcdea', 'a'),
            ('abcabc', 'abc'),
            ('abcdabc', 'abc'),
        ]
        for string, expected in test_tuples:
            self.assertEqual(self.solver.longestDupSubstring(string), expected)

    def test_overlapping(self):
        test_tuples = [
            ('banana', 'ana'),
            ('bannanna', 'anna'),
            ('aaaaa', 'aaaa')
        ]
        for string, expected in test_tuples:
            self.assertEqual(self.solver.longestDupSubstring(string), expected)
    
    if __name__ == '__main__':
        unittest.main()