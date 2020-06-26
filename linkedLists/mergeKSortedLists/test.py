import unittest
from merge_k_sorted_lists import Solution
from context import ListNode

class TestMergeSortedLists(unittest.TestCase):
    solver = Solution()
    def test_invalid_input(self):
        test_tuples = [
            (None, None),
            ([None], None),
        ]
        for list_of_lists, expected in test_tuples:
            self.assertEqual(self.solver.merge_sorted_lists(list_of_lists), expected)


    def test_some_invalid_input(self):
        test_tuples = [
            ([None, ListNode(1)], [1]),
            ([ListNode(3), ListNode(1), None, ListNode(-1), None], [-1, 1, 3]),
        ]
        for list_of_lists, expected in test_tuples:
            output = self.solver.merge_sorted_lists(list_of_lists)
            current = output
            for num in expected:
                self.assertEqual(current.val, num)
                current = current.next


    def test_single_node_lists(self):
        test_tuples = [
            ([ListNode(1), ListNode()], [0, 1]),
            ([ListNode(3), ListNode(1), ListNode(2), ListNode(-1)], [-1, 1, 2, 3]),
        ]
        for list_of_lists, expected in test_tuples:
            output = self.solver.merge_sorted_lists(list_of_lists)
            current = output
            for num in expected:
                self.assertEqual(current.val, num)
                current = current.next
    
    
    def test_multiple_node_lists(self):
        test_tuples = [
            (
                [
                    ListNode(1, ListNode(4, ListNode(5))),
                    ListNode(1, ListNode(3, ListNode(4))),
                    ListNode(2, ListNode(6))
                ],
                [1, 1, 2, 3, 4, 4, 5, 6]
            ),
        ]

        for list_of_lists, expected in test_tuples:
            output = self.solver.merge_sorted_lists(list_of_lists)
            current = output
            for num in expected:
                self.assertEqual(current.val, num)
                current = current.next

if __name__ == '__main__':
    unittest.main()