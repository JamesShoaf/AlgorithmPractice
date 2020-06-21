import unittest
from longest_duplicate_substring import Solution 

class TestLongestDuplicateSubstring(unittest.TestCase):
    solver = Solution()
    def test_no_duplicates(self):
        'it should return an empty string if there are no duplicate substrings'
        test_tuples = [
            ('', ''),
            ('abcd', ''),
        ]
        for string, expected in test_tuples:
            self.assertEqual(self.solver.longestDupSubstring(string), expected)
    
    def test_non_overlapping(self):
        'it should return the correct duplicate substring'
        test_tuples = [
            ('abcda', 'a'),
            ('abcdea', 'a'),
            ('abcabc', 'abc'),
            ('abcdabc', 'abc'),
        ]
        for string, expected in test_tuples:
            self.assertEqual(self.solver.longestDupSubstring(string), expected)

    def test_overlapping(self):
        'it should return the correct duplicate substring'
        test_tuples = [
            ('banana', 'ana'),
            ('bannanna', 'anna'),
            ('aaaaa', 'aaaa')
        ]
        for string, expected in test_tuples:
            self.assertEqual(self.solver.longestDupSubstring(string), expected)
    
if __name__ == '__main__':
    unittest.main()