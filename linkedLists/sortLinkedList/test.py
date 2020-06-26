import unittest
from sort_linked_list import Solution
from context import ListNode

class TestMergeSortedLists(unittest.TestCase):
    solver = Solution()
    def test_invalid_input(self):
        test_tuples = [
            (None, None),
            ([1, 2, 3], None),
        ]
        for list_of_lists, expected in test_tuples:
            self.assertEqual(self.solver.sort_linked_list(list_of_lists), expected)


    def test_unsorted_lists(self):
        test_tuples = [
            (ListNode(0), [0]),
            (ListNode(3, ListNode(1, ListNode(2, ListNode(-1)))), [-1, 1, 2, 3]),
            (ListNode(3, ListNode(1, ListNode(2, ListNode(-1, ListNode(0))))), [-1, 0, 1, 2, 3]),
        ]
        for list, expected in test_tuples:
            output = self.solver.sort_linked_list(list)
            current = output
            for num in expected:
                self.assertEqual(current.val, num)
                current = current.next

if __name__ == '__main__':
    unittest.main()