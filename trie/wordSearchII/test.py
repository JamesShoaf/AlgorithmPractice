import unittest
from word_search_II import Solution

class TestWordSearch(unittest.TestCase):
    solver = Solution()
    board = [
        ['o','a','a','n'],
        ['e','t','a','e'],
        ['i','h','k','r'],
        ['i','f','l','v'],
    ]

    def test_search(self):
        test_tuples = [
            (['oath', 'pea', 'eat', 'rain'],
            set(['oath', 'eat'])),
            (['oaanervlfiietakh'],
            set(['oaanervlfiietakh'])),
            (['oeiiieo'],
            set()),
        ]
        for word_list, expected_set in test_tuples:
            output_list = self.solver.search(self.board, word_list)
            self.assertEqual(len(output_list), len(expected_set))
            for word in output_list:
                self.assertTrue(word in expected_set)
                expected_set.remove(word)

if __name__ == '__main__':
    unittest.main()